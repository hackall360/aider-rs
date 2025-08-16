use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver};

use anyhow::Result;
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};

/// Handle for a running watcher returning filtered events.
pub struct WatcherHandle {
    _watcher: RecommendedWatcher,
    #[allow(dead_code)]
    pub rx: Receiver<Result<Event, notify::Error>>,
    #[allow(dead_code)]
    gitignore: Gitignore,
}

impl WatcherHandle {
    /// Wait for the next path change that isn't ignored.
    #[allow(dead_code)]
    pub fn next_path(&self) -> Option<PathBuf> {
        while let Ok(event) = self.rx.recv() {
            if let Ok(event) = event {
                for path in event.paths {
                    if !self
                        .gitignore
                        .matched_path_or_any_parents(&path, path.is_dir())
                        .is_ignore()
                    {
                        return Some(path);
                    }
                }
            }
        }
        None
    }
}

/// Create a new watcher rooted at `path` that respects `.gitignore` rules.
pub fn watch(path: &Path) -> Result<WatcherHandle> {
    let (tx, rx) = channel();
    let mut watcher = RecommendedWatcher::new(tx, notify::Config::default())?;
    watcher.watch(path, RecursiveMode::Recursive)?;

    let mut builder = GitignoreBuilder::new(path);
    builder.add(".gitignore");
    let gitignore = builder.build()?;

    Ok(WatcherHandle {
        _watcher: watcher,
        rx,
        gitignore,
    })
}
