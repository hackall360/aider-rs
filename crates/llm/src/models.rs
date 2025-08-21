use crate::openrouter::OpenRouterModelManager;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

pub struct ModelInfoManager {
    cache_dir: PathBuf,
    cache_file: PathBuf,
    content: Option<Value>,
    verify_ssl: bool,
    cache_loaded: bool,
    pub(crate) openrouter_manager: OpenRouterModelManager,
}

impl ModelInfoManager {
    const MODEL_INFO_URL: &'static str = "https://raw.githubusercontent.com/BerriAI/litellm/main/model_prices_and_context_window.json";
    const CACHE_TTL: Duration = Duration::from_secs(60 * 60 * 24);

    pub fn new() -> Self {
        let cache_dir = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".aider")
            .join("caches");
        let cache_file = cache_dir.join("model_prices_and_context_window.json");
        Self {
            cache_dir,
            cache_file,
            content: None,
            verify_ssl: true,
            cache_loaded: false,
            openrouter_manager: OpenRouterModelManager::new(),
        }
    }

    pub fn set_verify_ssl(&mut self, verify: bool) {
        self.verify_ssl = verify;
        self.openrouter_manager.set_verify_ssl(verify);
    }

    /// Create a manager with a custom OpenRouter manager (useful for tests).
    pub fn with_openrouter_manager(openrouter: OpenRouterModelManager) -> Self {
        let mut mgr = Self::new();
        mgr.openrouter_manager = openrouter;
        mgr
    }

    /// Override the cached content (primarily for tests).
    pub fn set_content(&mut self, content: Value) {
        self.content = Some(content);
        self.cache_loaded = true;
    }

    pub async fn get_model_info(&mut self, model: &str) -> Value {
        self.ensure_content().await;
        if let Some(content) = &self.content {
            if let Some(info) = content.get(model) {
                return info.clone();
            }
            if let Some((provider, name)) = model.split_once('/') {
                if let Some(info) = content.get(name) {
                    if info.get("litellm_provider").and_then(|v| v.as_str()) == Some(provider) {
                        return info.clone();
                    }
                }
            }
        }
        if model.starts_with("openrouter/") {
            if let Some(info) = self.openrouter_manager.get_model_info(model).await {
                return info;
            }
        }
        Value::Null
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
        if let Ok(resp) = client.get(Self::MODEL_INFO_URL).send().await {
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

pub struct Model {
    pub name: String,
    pub info: Value,
}

impl Model {
    pub async fn new(name: &str, manager: &mut ModelInfoManager) -> Self {
        let info = manager.get_model_info(name).await;
        Self {
            name: name.to_string(),
            info,
        }
    }
}
