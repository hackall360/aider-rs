use reqwest;
use thiserror::Error;
use tracing::info;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
}

pub async fn fetch(url: &str) -> Result<String, CoreError> {
    info!("fetching {url}");
    let resp = reqwest::get(url).await?;
    Ok(resp.text().await?)
}

pub fn ping() -> &'static str {
    "pong"
}
