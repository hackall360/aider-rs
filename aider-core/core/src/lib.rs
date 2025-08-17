use std::process::Command;
use thiserror::Error;
use tracing::info;

pub mod chat;
pub mod models;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("git error: {0}")]
    Git(String),
    #[error("invalid input: {0}")]
    Invalid(String),
}

pub async fn fetch(url: &str) -> Result<String, CoreError> {
    info!("fetching {url}");
    let resp = reqwest::get(url).await?;
    Ok(resp.text().await?)
}

pub fn ping() -> &'static str {
    "pong"
}

pub fn git(args: Vec<String>) -> Result<String, CoreError> {
    let output = Command::new("git")
        .args(&args)
        .output()
        .map_err(|e| CoreError::Git(e.to_string()))?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn repo_map() -> String {
    "repo map not implemented".to_string()
}

pub fn llm(prompt: String) -> String {
    format!("LLM response to: {prompt}")
}
