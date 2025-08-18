use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use tracing::instrument;

/// Configuration loaded from YAML files and environment variables.
#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Mirror of the `--verbose` flag.
    pub verbose: bool,
    /// API key for OpenAI models.
    pub openai_api_key: Option<String>,
    /// API key for Anthropic models.
    pub anthropic_api_key: Option<String>,
    /// Preferred edit format for generated patches.
    pub edit_format: Option<String>,
    /// Maximum tokens used for repository maps.
    pub map_tokens: Option<usize>,
    /// Automatically run lint commands after edits.
    pub auto_lint: bool,
    /// Automatically run tests after edits.
    pub auto_test: bool,
    /// Logging level passed to `tracing`.
    pub log_level: Option<String>,
    /// Show token and cost usage after each turn.
    pub show_usage: bool,
    /// Include reasoning tokens if the model supports them.
    pub reasoning_tokens: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            verbose: false,
            openai_api_key: None,
            anthropic_api_key: None,
            edit_format: None,
            map_tokens: None,
            auto_lint: false,
            auto_test: false,
            log_level: None,
            show_usage: false,
            reasoning_tokens: true,
        }
    }
}

impl Config {
    /// Load configuration from the provided path or the default config location.
    #[instrument]
    pub fn load(path: Option<&Path>) -> Result<Self> {
        let path = path
            .map(PathBuf::from)
            .or_else(default_config_path)
            .unwrap();

        let mut cfg = if path.exists() {
            let text = fs::read_to_string(&path)?;
            serde_yaml::from_str(&text)?
        } else {
            Config::default()
        };

        // Environment variable overrides
        if let Ok(val) = env::var("OPENAI_API_KEY") {
            cfg.openai_api_key = Some(val);
        }
        if let Ok(val) = env::var("ANTHROPIC_API_KEY") {
            cfg.anthropic_api_key = Some(val);
        }
        if let Ok(val) = env::var("AIDER_EDIT_FORMAT") {
            cfg.edit_format = Some(val);
        }
        if let Ok(val) = env::var("AIDER_MAP_TOKENS") {
            if let Ok(v) = val.parse() {
                cfg.map_tokens = Some(v);
            }
        }
        if let Ok(val) = env::var("AIDER_AUTO_LINT") {
            cfg.auto_lint = parse_bool(&val);
        }
        if let Ok(val) = env::var("AIDER_AUTO_TEST") {
            cfg.auto_test = parse_bool(&val);
        }
        if let Ok(val) = env::var("AIDER_LOG_LEVEL") {
            cfg.log_level = Some(val);
        }
        if let Ok(val) = env::var("AIDER_VERBOSE") {
            cfg.verbose = parse_bool(&val);
        }
        if let Ok(val) = env::var("AIDER_SHOW_USAGE") {
            cfg.show_usage = parse_bool(&val);
        }
        if let Ok(val) = env::var("AIDER_REASONING_TOKENS") {
            cfg.reasoning_tokens = parse_bool(&val);
        }

        Ok(cfg)
    }

    /// Save settings to the chosen path or default location.
    pub fn save(&self, path: Option<&Path>) -> Result<()> {
        let path = path
            .map(PathBuf::from)
            .or_else(default_config_path)
            .unwrap();

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let data = serde_yaml::to_string(self)?;
        fs::write(path, data)?;
        Ok(())
    }

    /// Directory used to store analytics, history, and other persistent files.
    pub fn data_dir() -> PathBuf {
        default_config_path()
            .and_then(|p| p.parent().map(Path::to_path_buf))
            .unwrap()
    }
}

fn parse_bool(val: &str) -> bool {
    matches!(val.to_lowercase().as_str(), "1" | "true" | "yes" | "on")
}

fn default_config_path() -> Option<PathBuf> {
    ProjectDirs::from("com", "aider", "aider-cli").map(|dir| dir.config_dir().join("config.yaml"))
}
