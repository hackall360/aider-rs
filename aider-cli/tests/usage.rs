use aider_cli::usage::UsageTracker;
use aider_llm::Usage;
use serde_json::json;

#[test]
fn reports_cost_per_turn() {
    let meta = json!({
        "test-model": {
            "input_cost_per_token": 0.001,
            "output_cost_per_token": 0.002
        }
    });
    let mut tracker = UsageTracker::new("test-model", meta);
    let msg1 = tracker.report(Usage { prompt_tokens: 10, completion_tokens: 5, cost: 0.0 });
    assert!(msg1.contains("tokens: 10 in, 5 out"));
    assert!(msg1.contains("cost $0.020000"));
    let msg2 = tracker.report(Usage { prompt_tokens: 20, completion_tokens: 10, cost: 0.0 });
    assert!(msg2.contains("total $0.040000"));
}
