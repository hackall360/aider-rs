use std::sync::{Arc, Mutex};
use std::time::Duration;

use bytes::Bytes;
use reqwest::Client;
use serde_json::{json, Value};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;

use crate::{ChatChunk, ModelProvider, Usage};

#[derive(Clone)]
pub struct GoProxyConfig {
    pub base_url: String,
    pub model: String,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
}

impl GoProxyConfig {
    pub fn from_env() -> Self {
        Self {
            base_url: std::env::var("GO_PROXY_URL")
                .unwrap_or_else(|_| "http://localhost:8080".into()),
            model: std::env::var("GO_PROXY_MODEL").unwrap_or_else(|_| "gpt-4o".into()),
            temperature: None,
            max_tokens: None,
        }
    }
}

#[derive(Clone)]
pub struct GoProxyProvider {
    client: Client,
    cfg: GoProxyConfig,
    usage: Arc<Mutex<Usage>>,
}

impl GoProxyProvider {
    pub fn new(cfg: GoProxyConfig) -> Self {
        Self {
            client: Client::new(),
            usage: Arc::new(Mutex::new(Usage::default())),
            cfg,
        }
    }

    fn build_body(&self, prompt: String) -> Value {
        let mut body = json!({
            "model": self.cfg.model,
            "messages": [ {"role": "user", "content": prompt} ],
        });
        if let Some(t) = self.cfg.temperature {
            body["temperature"] = json!(t);
        }
        if let Some(mt) = self.cfg.max_tokens {
            body["max_tokens"] = json!(mt);
        }
        body
    }

    async fn process_stream<S>(mut stream: S, tx: mpsc::Sender<ChatChunk>)
    where
        S: tokio_stream::Stream<Item = Result<Bytes, reqwest::Error>> + Unpin,
    {
        let mut buf = String::new();
        while let Some(chunk) = stream.next().await {
            match chunk {
                Ok(bytes) => {
                    buf.push_str(&String::from_utf8_lossy(&bytes));
                    while let Some(pos) = buf.find('\n') {
                        let line = buf[..pos].to_string();
                        buf = buf[pos + 1..].to_string();
                        if line.trim().is_empty() {
                            continue;
                        }
                        if let Ok(json) = serde_json::from_str::<Value>(&line) {
                            if let Some(token) = json.get("token").and_then(|t| t.as_str()) {
                                let _ = tx.send(ChatChunk::Token(token.to_string())).await;
                            }
                            if json.get("done").and_then(|d| d.as_bool()) == Some(true) {
                                return;
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

    async fn send_request(&self, prompt: String, tx: mpsc::Sender<ChatChunk>) {
        let url = format!("{}/complete", self.cfg.base_url.trim_end_matches('/'));
        let body = self.build_body(prompt);
        let mut attempt: u32 = 0;
        loop {
            attempt += 1;
            let res = self.client.post(&url).json(&body).send().await;
            match res {
                Ok(r) if r.status().is_success() => {
                    let stream = r.bytes_stream();
                    Self::process_stream(stream, tx).await;
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
                    if attempt >= 3 {
                        let _ = tx.send(ChatChunk::Token(format!("error: {}", e))).await;
                        break;
                    }
                    let backoff = 2u64.pow(attempt);
                    tokio::time::sleep(Duration::from_secs(backoff)).await;
                }
            }
        }
    }
}

impl ModelProvider for GoProxyProvider {
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
