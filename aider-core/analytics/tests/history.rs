use aider_analytics::Analytics;
use serde_json::json;
use tempfile::tempdir;

#[tokio::test]
async fn test_log_formatting_and_persistence() {
    let tmp = tempdir().unwrap();
    let dir = tmp.path().to_path_buf();
    let mut analytics = Analytics::new_with_dir("http://localhost", "key", Some(dir.clone()));
    analytics
        .event("test_event", json!({"foo": "bar"}))
        .await
        .unwrap();

    let history_path = dir.join("session_history.json");
    let contents = std::fs::read_to_string(&history_path).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&contents).unwrap();
    assert_eq!(parsed["events"][0]["event"], "test_event");
    assert_eq!(parsed["events"][0]["properties"]["foo"], "bar");

    let analytics2 = Analytics::new_with_dir("http://localhost", "key", Some(dir));
    assert_eq!(analytics2.events().len(), 1);
    assert_eq!(analytics2.events()[0].event, "test_event");
}

#[tokio::test]
async fn test_version_check_resets_history() {
    let tmp = tempdir().unwrap();
    let dir = tmp.path().to_path_buf();
    let path = dir.join("session_history.json");
    std::fs::write(&path, "{\"version\":999,\"events\":[]}").unwrap();
    let analytics = Analytics::new_with_dir("http://localhost", "key", Some(dir));
    assert_eq!(analytics.events().len(), 0);
}
