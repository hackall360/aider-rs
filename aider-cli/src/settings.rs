use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

/// Persistent user settings stored as TOML or YAML.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Settings {
    /// Mirror of the `--verbose` flag.
    pub verbose: bool,
}

impl Settings {
    /// Load configuration from the provided path or the default config location.
    pub fn load_or_default(path: Option<&Path>) -> Result<Self> {
        let path = path
            .map(PathBuf::from)
            .or_else(default_config_path)
            .unwrap();

        if path.exists() {
            let text = fs::read_to_string(&path)?;
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("toml");
            match ext {
                "yaml" | "yml" => Ok(serde_yaml::from_str(&text)?),
                _ => Ok(toml::from_str(&text)?),
            }
        } else {
            Ok(Self::default())
        }
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

        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("toml");
        let data = match ext {
            "yaml" | "yml" => serde_yaml::to_string(self)?,
            _ => toml::to_string(self)?,
        };
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

fn default_config_path() -> Option<PathBuf> {
    ProjectDirs::from("com", "aider", "aider-cli").map(|dir| dir.config_dir().join("settings.toml"))
}
