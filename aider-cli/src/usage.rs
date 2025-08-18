use aider_llm::Usage;
use serde_json::Value;

/// Tracks token usage and running cost per turn.
pub struct UsageTracker {
    model: String,
    metadata: Value,
    last: Usage,
    total_cost: f64,
}

impl UsageTracker {
    /// Create a new tracker for the given model and metadata.
    pub fn new(model: &str, metadata: Value) -> Self {
        Self {
            model: model.to_string(),
            metadata,
            last: Usage::default(),
            total_cost: 0.0,
        }
    }

    /// Report usage for a turn, returning a formatted message.
    pub fn report(&mut self, usage: Usage) -> String {
        let delta_prompt = usage.prompt_tokens.saturating_sub(self.last.prompt_tokens);
        let delta_completion =
            usage.completion_tokens.saturating_sub(self.last.completion_tokens);
        self.last = usage;

        let (input_cost, output_cost) = self
            .metadata
            .get(&self.model)
            .and_then(|m| {
                Some((
                    m.get("input_cost_per_token")?.as_f64().unwrap_or(0.0),
                    m.get("output_cost_per_token")?.as_f64().unwrap_or(0.0),
                ))
            })
            .unwrap_or((0.0, 0.0));

        let cost = delta_prompt as f64 * input_cost + delta_completion as f64 * output_cost;
        self.total_cost += cost;
        format!(
            "tokens: {} in, {} out; cost ${:.6} (total ${:.6})",
            delta_prompt, delta_completion, cost, self.total_cost
        )
    }
}
