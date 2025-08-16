use moka::sync::Cache;
use once_cell::sync::Lazy;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::Duration;

/// Information about an LLM model available from OpenRouter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    /// Model identifier.
    pub id: String,
    /// Optional pricing information as returned by the API.
    #[serde(default)]
    pub pricing: Value,
}

// Cache the list of models for one hour.
static MODEL_CACHE: Lazy<Cache<&'static str, Vec<Model>>> = Lazy::new(|| {
    Cache::builder()
        .time_to_live(Duration::from_secs(60 * 60))
        .build()
});

/// Fetch the list of models from OpenRouter, using an in-memory cache.
pub async fn fetch_models() -> Result<Vec<Model>, reqwest::Error> {
    if let Some(models) = MODEL_CACHE.get(&"all") {
        return Ok(models);
    }

    let client = Client::new();
    let resp = client
        .get("https://openrouter.ai/api/v1/models")
        .send()
        .await?;
    let data: Value = resp.json().await?;
    let models: Vec<Model> =
        serde_json::from_value(data.get("data").cloned().unwrap_or(Value::Null))
            .unwrap_or_default();
    MODEL_CACHE.insert("all", models.clone());
    Ok(models)
}
