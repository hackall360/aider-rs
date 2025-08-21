use aider_llm::models::ModelInfoManager;
use aider_llm::openrouter::OpenRouterModelManager;
use serde_json::json;

#[tokio::test]
async fn openrouter_model_info_from_cache() {
    let content = json!({
        "data": [{
            "id": "a/b",
            "context_length": 100,
            "pricing": {"prompt": "0.1", "completion": "0.2"}
        }]
    });
    let or = OpenRouterModelManager::with_content(content);
    let mut mgr = ModelInfoManager::with_openrouter_manager(or);
    let info = mgr.get_model_info("openrouter/a/b").await;
    assert_eq!(info.get("max_tokens").and_then(|v| v.as_u64()), Some(100));
    assert_eq!(
        info.get("litellm_provider").and_then(|v| v.as_str()),
        Some("openrouter")
    );
}

#[test]
fn urls_constant() {
    assert_eq!(aider_llm::urls::WEBSITE, "https://aider.chat/");
}
