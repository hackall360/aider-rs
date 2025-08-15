use flutter_rust_bridge::frb;

#[frb]
pub fn llm(prompt: String) -> String {
    aider_core::llm(prompt)
}

#[frb]
pub fn repo_map() -> String {
    aider_core::repo_map()
}
