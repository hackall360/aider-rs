use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use bytes::Bytes;
use once_cell::sync::Lazy;
use reqwest::Client;
use serde_json::{json, Value};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;

use crate::{ChatChunk, ModelProvider, Usage};

static MODEL_ALIASES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("sonnet", "claude-3-5-sonnet");
    m.insert("haiku", "claude-3-5-haiku");
    m
});

fn resolve_model(model: &str) -> String {
    MODEL_ALIASES
        .get(model)
        .copied()
        .unwrap_or(model)
        .to_string()
}

#[derive(Clone)]
pub struct AnthropicConfig {
    pub api_key: String,
    pub model: String,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub system_prompt: Option<String>,
}

impl AnthropicConfig {
    pub fn from_env(model: &str) -> Option<Self> {
        let api_key = std::env::var("ANTHROPIC_API_KEY").ok()?;
        Some(Self {
            api_key,
            model: resolve_model(model),
            temperature: None,
            max_tokens: None,
            system_prompt: None,
        })
    }
}

#[derive(Clone)]
pub struct AnthropicProvider {
    client: Client,
    cfg: AnthropicConfig,
    usage: Arc<Mutex<Usage>>,
    base_url: String,
}

impl AnthropicProvider {
    pub fn new(cfg: AnthropicConfig) -> Self {
        Self {
            client: Client::new(),
            usage: Arc::new(Mutex::new(Usage::default())),
            base_url: "https://api.anthropic.com/v1/messages".into(),
            cfg,
        }
    }

    #[cfg(test)]
    fn with_base_url(mut self, url: &str) -> Self {
        self.base_url = url.to_string();
        self
    }

    async fn send_request(&self, prompt: String, tx: mpsc::Sender<ChatChunk>) {
        let mut attempt: u32 = 0;
        let body = self.build_body(prompt);
        loop {
            attempt += 1;
            let res = self
                .client
                .post(&self.base_url)
                .header("x-api-key", &self.cfg.api_key)
                .header("anthropic-version", "2023-06-01")
                .json(&body)
                .send()
                .await;

            match res {
                Ok(r) if r.status().is_success() => {
                    let stream = r.bytes_stream();
                    Self::process_stream(stream, tx, self.usage.clone()).await;
                    break;
                }
                Ok(r) if r.status().as_u16() == 429 || r.status().is_server_error() => {
                    if attempt >= 3 {
                        let _ = tx
                            .send(ChatChunk::Token("error: too many retries".into()))
                            .await;
                        break;
                    }
                    let backoff = 2u64.pow(attempt);
                    tokio::time::sleep(Duration::from_secs(backoff)).await;
                }
                Ok(r) => {
                    let msg = format!("error: HTTP {}", r.status());
                    let _ = tx.send(ChatChunk::Token(msg)).await;
                    break;
                }
                Err(e) => {
                    let _ = tx.send(ChatChunk::Token(format!("error: {}", e))).await;
                    break;
                }
            }
        }
    }

    fn build_body(&self, prompt: String) -> Value {
        let mut body = json!({
            "model": self.cfg.model,
            "messages": [{"role": "user", "content": prompt}],
            "stream": true,
        });
        if let Some(sys) = &self.cfg.system_prompt {
            body["system"] = json!(sys);
        }
        if let Some(t) = self.cfg.temperature {
            body["temperature"] = json!(t);
        }
        if let Some(mt) = self.cfg.max_tokens {
            body["max_tokens"] = json!(mt);
        }
        body
    }

    async fn process_stream<S>(mut stream: S, tx: mpsc::Sender<ChatChunk>, usage: Arc<Mutex<Usage>>)
    where
        S: tokio_stream::Stream<Item = Result<Bytes, reqwest::Error>> + Unpin,
    {
        let mut buf = String::new();
        while let Some(chunk) = stream.next().await {
            match chunk {
                Ok(bytes) => {
                    buf.push_str(&String::from_utf8_lossy(&bytes));
                    while let Some(pos) = buf.find("\n\n") {
                        let line = buf[..pos].to_string();
                        buf = buf[pos + 2..].to_string();
                        if let Some(data) = line.strip_prefix("data: ") {
                            if data.trim() == "[DONE]" {
                                return;
                            }
                            if let Ok(json) = serde_json::from_str::<Value>(data) {
                                if let Some(t) = json.get("type").and_then(|v| v.as_str()) {
                                    match t {
                                        "content_block_delta" => {
                                            if let Some(text) = json
                                                .get("delta")
                                                .and_then(|d| d.get("text"))
                                                .and_then(|v| v.as_str())
                                            {
                                                let _ = tx
                                                    .send(ChatChunk::Token(text.to_string()))
                                                    .await;
                                            }
                                        }
                                        "message_delta" => {
                                            if let Some(u) = json.get("usage") {
                                                let mut usage = usage.lock().unwrap();
                                                usage.prompt_tokens = u
                                                    .get("input_tokens")
                                                    .and_then(|v| v.as_u64())
                                                    .unwrap_or(0)
                                                    as u32;
                                                usage.completion_tokens = u
                                                    .get("output_tokens")
                                                    .and_then(|v| v.as_u64())
                                                    .unwrap_or(0)
                                                    as u32;
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    let _ = tx.send(ChatChunk::Token(format!("error: {}", e))).await;
                    return;
                }
            }
        }
    }
}

impl ModelProvider for AnthropicProvider {
    fn chat(&self, prompt: String) -> ReceiverStream<ChatChunk> {
        let (tx, rx) = mpsc::channel(32);
        let this = self.clone();
        tokio::spawn(async move {
            this.send_request(prompt, tx).await;
        });
        ReceiverStream::new(rx)
    }

    fn usage(&self) -> Usage {
        self.usage.lock().unwrap().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_stream::StreamExt;

    #[tokio::test]
    async fn parses_sse_stream() {
        let lines = vec![
            "data: {\"type\":\"content_block_delta\",\"delta\":{\"text\":\"Hi\"}}\n\n".to_string(),
            "data: {\"type\":\"content_block_delta\",\"delta\":{\"text\":\"!\"}}\n\n".to_string(),
            "data: [DONE]\n\n".to_string(),
        ];
        let stream = tokio_stream::iter(
            lines
                .into_iter()
                .map(|l| Ok::<Bytes, reqwest::Error>(Bytes::from(l))),
        );
        let (tx, rx) = mpsc::channel(10);
        let usage = Arc::new(Mutex::new(Usage::default()));
        AnthropicProvider::process_stream(stream, tx, usage).await;
        let collected: Vec<String> = ReceiverStream::new(rx)
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .filter_map(|c| match c {
                ChatChunk::Token(t) => Some(t),
                _ => None,
            })
            .collect();
        assert_eq!(collected.join(""), "Hi!");
    }
}
