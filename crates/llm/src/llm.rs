use once_cell::sync::Lazy;
use reqwest::Client;

pub const AIDER_SITE_URL: &str = "https://aider.chat";
pub const AIDER_APP_NAME: &str = "Aider";

/// Lazily initialized HTTP client used for LLM requests.
pub static CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .user_agent(AIDER_APP_NAME)
        .build()
        .expect("failed to build reqwest client")
});
