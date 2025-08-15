use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Basic analytics storage similar to the Python `analytics.py` module.
#[derive(Serialize, Deserialize, Default)]
pub struct Analytics {
    events: Vec<String>,
    path: PathBuf,
}

impl Analytics {
    pub fn new(path: PathBuf) -> Self {
        let events = if path.exists() {
            serde_yaml::from_str(&fs::read_to_string(&path).unwrap_or_default()).unwrap_or_default()
        } else {
            Vec::new()
        };
        Self { events, path }
    }

    /// Record a new analytics event.
    pub fn record(&mut self, event: &str) {
        self.events.push(event.to_string());
        if let Ok(serialized) = serde_yaml::to_string(&self.events) {
            let _ = fs::write(&self.path, serialized);
        }
    }
}

