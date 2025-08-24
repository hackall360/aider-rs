use std::collections::HashMap;

use anyhow::Result;
use serde::Deserialize;

/// Information about a language model.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct ModelInfo {
    /// Maximum tokens accepted as input.
    pub max_input_tokens: Option<u32>,
    /// Maximum tokens that can be generated.
    pub max_output_tokens: Option<u32>,
    /// Cost per token for input.
    pub input_cost_per_token: Option<f64>,
    /// Cost per token for output.
    pub output_cost_per_token: Option<f64>,
}

/// Manager which fetches model information from the OpenRouter API.
#[derive(Default)]
pub struct ModelInfoManager {
    cache: HashMap<String, ModelInfo>,
    client: reqwest::Client,
    base_url: String,
}

impl ModelInfoManager {
    /// Create a new manager using the default OpenRouter endpoint.
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            client: reqwest::Client::new(),
            base_url: "https://openrouter.ai/api/v1/models".to_string(),
        }
    }

    /// Create a new manager pointing at a custom endpoint. Useful for tests.
    pub fn with_base_url(base_url: String) -> Self {
        Self {
            base_url,
            ..Self::new()
        }
    }

    /// Fetch model information, caching the result locally.
    pub async fn get_model_info(&mut self, model: &str) -> Result<ModelInfo> {
        if let Some(info) = self.cache.get(model) {
            return Ok(info.clone());
        }
        let url = format!("{}/{}", self.base_url, model);
        let resp = self.client.get(&url).send().await?.error_for_status()?;
        let info: ModelInfo = resp.json().await?;
        self.cache.insert(model.to_string(), info.clone());
        Ok(info)
    }

    /// Return model names that fuzzily match the provided search term.
    pub fn fuzzy_match_models(&self, search: &str) -> Vec<String> {
        let search = search.to_lowercase();
        let mut matches: Vec<String> = self
            .cache
            .keys()
            .filter(|m| m.to_lowercase().contains(&search))
            .cloned()
            .collect();
        matches.sort();
        matches
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;

    #[tokio::test]
    async fn fetches_model_info_from_api() {
        let server = MockServer::start();
        let _mock = server.mock(|when, then| {
            when.method(GET).path("/test-model");
            then.status(200)
                .header("content-type", "application/json")
                .body(r#"{
                    "max_input_tokens": 1024,
                    "max_output_tokens": 512,
                    "input_cost_per_token": 0.0001,
                    "output_cost_per_token": 0.0002
                }"#);
        });
        let mut mgr = ModelInfoManager::with_base_url(server.url(""));
        let info = mgr.get_model_info("test-model").await.unwrap();
        assert_eq!(info.max_input_tokens, Some(1024));
        assert_eq!(info.max_output_tokens, Some(512));
        assert_eq!(info.input_cost_per_token, Some(0.0001));
        assert_eq!(info.output_cost_per_token, Some(0.0002));
    }

    #[test]
    fn matches_models_by_substring() {
        let mut mgr = ModelInfoManager::new();
        mgr.cache.insert("gpt-4".into(), ModelInfo::default());
        mgr.cache.insert("gpt-3.5".into(), ModelInfo::default());
        let matches = mgr.fuzzy_match_models("gpt");
        assert_eq!(matches, vec!["gpt-3.5".to_string(), "gpt-4".to_string()]);
    }
}

