use serde_json::{json, Value};
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

fn cost_per_token(val: Option<&str>) -> Option<f64> {
    match val {
        Some("0") => Some(0.0),
        Some(v) => v.parse().ok(),
        None => None,
    }
}

pub struct OpenRouterModelManager {
    cache_dir: PathBuf,
    cache_file: PathBuf,
    pub(crate) content: Option<Value>,
    verify_ssl: bool,
    cache_loaded: bool,
}

impl OpenRouterModelManager {
    const MODELS_URL: &'static str = "https://openrouter.ai/api/v1/models";
    const CACHE_TTL: Duration = Duration::from_secs(60 * 60 * 24);

    pub fn new() -> Self {
        let cache_dir = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".aider")
            .join("caches");
        let cache_file = cache_dir.join("openrouter_models.json");
        Self {
            cache_dir,
            cache_file,
            content: None,
            verify_ssl: true,
            cache_loaded: false,
        }
    }

    pub fn set_verify_ssl(&mut self, verify: bool) {
        self.verify_ssl = verify;
    }

    /// Create a manager with predefined content (useful for tests).
    pub fn with_content(content: Value) -> Self {
        let mut mgr = Self::new();
        mgr.content = Some(content);
        mgr.cache_loaded = true;
        mgr
    }

    pub async fn get_model_info(&mut self, model: &str) -> Option<Value> {
        self.ensure_content().await;
        let content = self.content.as_ref()?;
        let route = self.strip_prefix(model);
        let mut candidates = vec![route.to_string()];
        if let Some((base, _)) = route.split_once(':') {
            candidates.push(base.to_string());
        }
        let data = content.get("data")?.as_array()?;
        let record = data.iter().find(|item| {
            item.get("id")
                .and_then(|v| v.as_str())
                .map(|id| candidates.iter().any(|c| c == id))
                .unwrap_or(false)
        })?;
        let context_len = record
            .get("top_provider")
            .and_then(|tp| tp.get("context_length"))
            .or_else(|| record.get("context_length"))
            .and_then(|v| v.as_u64());
        let pricing = record
            .get("pricing")
            .and_then(|v| v.as_object())
            .cloned()
            .unwrap_or_default();
        let prompt = pricing.get("prompt").and_then(|v| v.as_str());
        let completion = pricing.get("completion").and_then(|v| v.as_str());
        Some(json!({
            "max_input_tokens": context_len,
            "max_tokens": context_len,
            "max_output_tokens": context_len,
            "input_cost_per_token": cost_per_token(prompt),
            "output_cost_per_token": cost_per_token(completion),
            "litellm_provider": "openrouter",
        }))
    }

    fn strip_prefix<'a>(&self, model: &'a str) -> &'a str {
        model.strip_prefix("openrouter/").unwrap_or(model)
    }

    async fn ensure_content(&mut self) {
        if !self.cache_loaded {
            self.load_cache();
        }
        if self.content.is_none() {
            self.update_cache().await;
        }
    }

    fn load_cache(&mut self) {
        if self.cache_loaded {
            return;
        }
        if let Ok(metadata) = fs::metadata(&self.cache_file) {
            if let Ok(modified) = metadata.modified() {
                if let Ok(age) = SystemTime::now().duration_since(modified) {
                    if age < Self::CACHE_TTL {
                        if let Ok(text) = fs::read_to_string(&self.cache_file) {
                            if let Ok(json) = serde_json::from_str(&text) {
                                self.content = Some(json);
                            }
                        }
                    }
                }
            }
        }
        self.cache_loaded = true;
    }

    async fn update_cache(&mut self) {
        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(!self.verify_ssl)
            .build()
            .expect("build client");
        if let Ok(resp) = client.get(Self::MODELS_URL).send().await {
            if resp.status().is_success() {
                if let Ok(json) = resp.json::<Value>().await {
                    self.content = Some(json.clone());
                    let _ = fs::create_dir_all(&self.cache_dir);
                    let _ = fs::write(
                        &self.cache_file,
                        serde_json::to_string_pretty(&json).unwrap(),
                    );
                }
            }
        }
    }
}
