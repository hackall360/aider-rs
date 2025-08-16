use futures_util::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::CoreError;

/// A single chat message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

/// Ensure that a list of chat messages is non-empty and each message has content.
pub fn sanity_check(messages: &[ChatMessage]) -> Result<(), CoreError> {
    if messages.is_empty() {
        return Err(CoreError::Invalid("no messages".into()));
    }
    if messages.iter().any(|m| m.content.trim().is_empty()) {
        return Err(CoreError::Invalid("empty message".into()));
    }
    Ok(())
}

/// Stream chat completions from OpenRouter as a concatenated String.
pub async fn chat(messages: &[ChatMessage]) -> Result<String, CoreError> {
    sanity_check(messages)?;
    let client = Client::new();
    let resp = client
        .post("https://openrouter.ai/api/v1/chat/completions")
        .json(&json!({
            "model": "gpt-3.5-turbo",
            "messages": messages,
            "stream": true
        }))
        .send()
        .await?;

    let mut stream = resp.bytes_stream();
    let mut out = String::new();
    while let Some(chunk) = stream.next().await {
        let bytes = chunk?;
        out.push_str(&String::from_utf8_lossy(&bytes));
    }
    Ok(out)
}
