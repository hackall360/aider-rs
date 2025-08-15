use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Simple command history implementation mirroring `history.py`.
#[derive(Serialize, Deserialize, Default)]
pub struct History {
    entries: Vec<String>,
    path: PathBuf,
}

impl History {
    pub fn new(path: PathBuf) -> Self {
        let entries = if path.exists() {
            serde_yaml::from_str(&fs::read_to_string(&path).unwrap_or_default()).unwrap_or_default()
        } else {
            Vec::new()
        };
        Self { entries, path }
    }

    /// Add an entry to the history and persist it.
    pub fn add(&mut self, entry: String) {
        self.entries.push(entry);
        if let Ok(serialized) = serde_yaml::to_string(&self.entries) {
            let _ = fs::write(&self.path, serialized);
        }
    }
}

