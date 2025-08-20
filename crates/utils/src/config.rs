use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};
use directories::BaseDirs;
use serde::{Deserialize, Serialize};

/// Configuration loaded from YAML files and environment variables.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbose: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openai_api_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anthropic_api_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_tokens: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_lint: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_test: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_usage: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_tokens: Option<bool>,
    // New fields for layered configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_budget: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lint_cmd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_cmd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coding_conventions: Option<String>,
}

impl Config {
    /// Merge another config into this one, overriding fields that are `Some`.
    pub fn merge(&mut self, other: Config) {
        macro_rules! merge_field {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        merge_field!(verbose);
        merge_field!(openai_api_key);
        merge_field!(anthropic_api_key);
        merge_field!(edit_format);
        merge_field!(map_tokens);
        merge_field!(auto_lint);
        merge_field!(auto_test);
        merge_field!(log_level);
        merge_field!(show_usage);
        merge_field!(reasoning_tokens);
        merge_field!(provider);
        merge_field!(token_budget);
        merge_field!(chat_mode);
        merge_field!(lint_cmd);
        merge_field!(test_cmd);
        merge_field!(coding_conventions);
    }

    /// Load configuration from global and project files with environment overrides.
    pub fn load(project_path: Option<&Path>) -> Result<Self> {
        let mut cfg = Config::default();

        if let Some(path) = global_config_path() {
            if path.exists() {
                merge_file(&mut cfg, &path)?;
            }
        }

        let proj_path = project_path.map(PathBuf::from).unwrap_or_else(|| {
            project_config_path().unwrap_or_else(|_| PathBuf::from(".aider.yaml"))
        });
        if proj_path.exists() {
            merge_file(&mut cfg, &proj_path)?;
        }

        cfg.merge(env_overrides());

        Ok(cfg)
    }

    /// Save configuration to the chosen path or global default.
    pub fn save(&self, path: Option<&Path>) -> Result<()> {
        let path = path.map(PathBuf::from).or_else(global_config_path).unwrap();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let data = serde_yaml::to_string(self)?;
        fs::write(&path, data)?;
        Ok(())
    }

    /// Directory used to store analytics, history, and other persistent files.
    pub fn data_dir() -> PathBuf {
        global_dir()
    }
}

fn merge_file(cfg: &mut Config, path: &Path) -> Result<()> {
    let text =
        fs::read_to_string(path).with_context(|| format!("unable to read {}", path.display()))?;
    match serde_yaml::from_str::<Config>(&text) {
        Ok(parsed) => {
            cfg.merge(parsed);
            Ok(())
        }
        Err(err) => {
            if let Some(loc) = err.location() {
                Err(anyhow!(
                    "{}:{}:{} {}",
                    path.display(),
                    loc.line(),
                    loc.column(),
                    err
                ))
            } else {
                Err(anyhow!("{}: {}", path.display(), err))
            }
        }
    }
}

fn env_overrides() -> Config {
    let mut cfg = Config::default();
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
        cfg.auto_lint = Some(parse_bool(&val));
    }
    if let Ok(val) = env::var("AIDER_AUTO_TEST") {
        cfg.auto_test = Some(parse_bool(&val));
    }
    if let Ok(val) = env::var("AIDER_LOG_LEVEL") {
        cfg.log_level = Some(val);
    }
    if let Ok(val) = env::var("AIDER_VERBOSE") {
        cfg.verbose = Some(parse_bool(&val));
    }
    if let Ok(val) = env::var("AIDER_SHOW_USAGE") {
        cfg.show_usage = Some(parse_bool(&val));
    }
    if let Ok(val) = env::var("AIDER_REASONING_TOKENS") {
        cfg.reasoning_tokens = Some(parse_bool(&val));
    }
    cfg
}

fn parse_bool(val: &str) -> bool {
    matches!(val.to_lowercase().as_str(), "1" | "true" | "yes" | "on")
}

fn global_dir() -> PathBuf {
    BaseDirs::new()
        .map(|d| d.home_dir().join(".aider"))
        .unwrap_or_else(|| PathBuf::from(".aider"))
}

fn global_config_path() -> Option<PathBuf> {
    Some(global_dir().join("aider.yaml"))
}

fn project_config_path() -> Result<PathBuf> {
    Ok(std::env::current_dir()?.join(".aider.yaml"))
}
