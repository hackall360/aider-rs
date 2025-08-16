use std::fs;
use std::path::Path;

use anyhow::Result;
use serde_json::Value as JsonValue;
use serde_yaml::Value as YamlValue;
use toml::Value as TomlValue;

/// Load a JSON resource from disk.
pub fn load_json<P: AsRef<Path>>(path: P) -> Result<JsonValue> {
    let data = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&data)?)
}

/// Load a YAML resource from disk.
pub fn load_yaml<P: AsRef<Path>>(path: P) -> Result<YamlValue> {
    let data = fs::read_to_string(path)?;
    Ok(serde_yaml::from_str(&data)?)
}

/// Load a TOML resource from disk.
#[allow(dead_code)]
pub fn load_toml<P: AsRef<Path>>(path: P) -> Result<TomlValue> {
    let data = fs::read_to_string(path)?;
    Ok(toml::from_str(&data)?)
}

/// Load a plain text prompt template.
pub fn load_prompt<P: AsRef<Path>>(path: P) -> Result<String> {
    Ok(fs::read_to_string(path)?)
}
