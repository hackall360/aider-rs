use serde::Serialize;
use serde_json::Value;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

#[derive(Clone, Debug, Default, Serialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub cost: f32,
}

#[derive(Clone, Debug, Serialize)]
pub struct ToolCall {
    pub name: String,
    pub arguments: Value,
}

#[derive(Clone, Debug, Serialize)]
pub enum ChatChunk {
    Token(String),
    ToolCall(ToolCall),
}

pub trait ModelProvider: Send + Sync {
    fn chat(&self, prompt: String) -> ReceiverStream<ChatChunk>;
    fn usage(&self) -> Usage;
}

pub mod anthropic;
pub mod go_proxy;
pub mod llm;
pub mod models;
pub mod openai;
pub mod openrouter;
pub mod sendchat;
pub mod urls;

pub mod mock {
    use super::*;

    #[derive(Clone, Default)]
    pub struct MockProvider {
        tokens: Option<Vec<String>>,
        usage: Usage,
    }

    impl MockProvider {
        pub fn new_with_tokens(tokens: Vec<String>) -> Self {
            Self {
                usage: Usage {
                    prompt_tokens: 0,
                    completion_tokens: tokens.len() as u32,
                    cost: 0.0,
                },
                tokens: Some(tokens),
            }
        }
    }

    impl ModelProvider for MockProvider {
        fn chat(&self, prompt: String) -> ReceiverStream<ChatChunk> {
            let tokens: Vec<String> = match &self.tokens {
                Some(ts) => ts.clone(),
                None => prompt.chars().map(|c| c.to_string()).collect(),
            };
            let (tx, rx) = mpsc::channel(32);
            tokio::spawn(async move {
                for token in tokens {
                    if tx.send(ChatChunk::Token(token)).await.is_err() {
                        break;
                    }
                }
            });
            ReceiverStream::new(rx)
        }

        fn usage(&self) -> Usage {
            self.usage.clone()
        }
    }
}

use once_cell::sync::Lazy;
use std::collections::HashMap;

type Factory = fn() -> Box<dyn ModelProvider>;

static REGISTRY: Lazy<HashMap<&'static str, Factory>> = Lazy::new(|| {
    let mut m: HashMap<&'static str, Factory> = HashMap::new();
    m.insert("mock", || Box::new(mock::MockProvider::default()));
    m.insert("mock2", || Box::new(mock::MockProvider::default()));
    m.insert("go-proxy", || {
        Box::new(go_proxy::GoProxyProvider::new(
            go_proxy::GoProxyConfig::from_env(),
        ))
    });
    m
});

/// Return a provider for the given model name or alias.
pub fn get_provider(name: &str) -> Option<Box<dyn ModelProvider>> {
    REGISTRY.get(name).map(|f| f())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_stream::StreamExt;

    #[tokio::test]
    async fn mock_provider_streams_tokens() {
        let provider = mock::MockProvider::new_with_tokens(vec!["a".into(), "b".into()]);
        let stream = provider.chat("hello".into());
        let chunks: Vec<ChatChunk> = stream.collect().await;
        let tokens: Vec<String> = chunks
            .into_iter()
            .filter_map(|c| match c {
                ChatChunk::Token(t) => Some(t),
                _ => None,
            })
            .collect();
        assert_eq!(tokens, vec!["a", "b"]);
    }
}
