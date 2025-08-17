use aider_analytics::Analytics;
use flutter_rust_bridge::frb;
use serde_json::Value;

#[frb]
pub fn llm(prompt: String) -> String {
    aider_core::llm(prompt)
}

#[frb]
pub fn repo_map() -> String {
    aider_core::repo_map()
}

#[frb]
pub fn voice_record() -> String {
    aider_core::voice::record().unwrap_or_else(|e| e.to_string())
}

#[frb]
pub fn git(command: String) -> String {
    let args = command.split_whitespace().map(|s| s.to_string()).collect();
    aider_core::git(args).unwrap_or_else(|e| e.to_string())
}

#[frb]
pub async fn scrape_url(url: String) -> String {
    aider_core::scrape::scrape_url(&url)
        .await
        .unwrap_or_else(|e| e.to_string())
}

#[frb]
pub async fn analytics_event(event: String, properties: String) -> bool {
    let props: Value = serde_json::from_str(&properties).unwrap_or(Value::Null);
    let host = std::env::var("POSTHOG_HOST").unwrap_or_else(|_| "https://us.i.posthog.com".into());
    let api_key = std::env::var("POSTHOG_PROJECT_API_KEY")
        .unwrap_or_else(|_| "phc_99T7muzafUMMZX15H8XePbMSreEUzahHbtWjy3l5Qbv".into());
    let analytics = Analytics::new(&host, &api_key);
    analytics.event(&event, props).await.is_ok()
}
