use reqwest::Client;
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Default)]
struct KeyData {
    #[serde(default)]
    is_free_tier: bool,
}

#[derive(Deserialize, Default)]
struct KeyResponse {
    #[serde(default)]
    data: Option<KeyData>,
}

/// Check if the provided OpenRouter key belongs to a free tier account.
/// Defaults to `true` if the request fails or the field is missing.
pub async fn check_openrouter_tier(api_key: &str) -> bool {
    let client = Client::new();
    let resp = client
        .get("https://openrouter.ai/api/v1/auth/key")
        .bearer_auth(api_key)
        .send()
        .await;
    if let Ok(resp) = resp {
        if let Ok(parsed) = resp.json::<KeyResponse>().await {
            return parsed
                .data
                .map(|d| d.is_free_tier)
                .unwrap_or(true);
        }
    }
    true
}

/// Try to select a default model based on available API keys.
/// Returns the model name or `None` if no suitable key is found.
pub async fn try_to_select_default_model() -> Option<String> {
    if let Ok(key) = env::var("OPENROUTER_API_KEY") {
        let free = check_openrouter_tier(&key).await;
        if free {
            return Some("openrouter/deepseek/deepseek-r1:free".into());
        } else {
            return Some("openrouter/anthropic/claude-sonnet-4".into());
        }
    }

    let pairs = [
        ("ANTHROPIC_API_KEY", "sonnet"),
        ("DEEPSEEK_API_KEY", "deepseek"),
        ("OPENAI_API_KEY", "gpt-4o"),
        ("GEMINI_API_KEY", "gemini/gemini-2.5-pro-exp-03-25"),
        (
            "VERTEXAI_PROJECT",
            "vertex_ai/gemini-2.5-pro-exp-03-25",
        ),
    ];

    for (env_key, model) in pairs {
        if env::var(env_key).is_ok() {
            return Some(model.to_string());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn selects_model_from_env() {
        env::set_var("OPENAI_API_KEY", "test");
        let model = try_to_select_default_model().await;
        env::remove_var("OPENAI_API_KEY");
        assert_eq!(model, Some("gpt-4o".to_string()));
    }
}

