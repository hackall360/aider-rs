use std::process::Command;
use thiserror::Error;
use tracing::info;

pub mod chat;
pub mod models;
pub mod scrape;
pub mod voice;
pub mod coders;
pub mod repo;
pub mod repomap;
pub mod watch;

pub use coders::{system_prompt, CoderKind};

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("git error: {0}")]
    Git(String),
    #[error("invalid input: {0}")]
    Invalid(String),
    #[error("audio error: {0}")]
    Audio(String),
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

pub fn repo_map() -> Result<String, CoreError> {
    let repo = repo::Repo::open(".")
        .map_err(|e| CoreError::Git(e.to_string()))?;
    repomap::build(&repo).map_err(|e| CoreError::Invalid(e.to_string()))
}

pub async fn watch_repo() -> Result<Vec<String>, CoreError> {
    let repo = repo::Repo::open(".")
        .map_err(|e| CoreError::Git(e.to_string()))?;
    watch::watch_once(repo)
        .await
        .map_err(|e| CoreError::Invalid(e.to_string()))
}

pub fn llm(prompt: String) -> String {
    format!("LLM response to: {prompt}")
}
