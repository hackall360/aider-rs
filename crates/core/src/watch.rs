use std::path::PathBuf;
use std::time::Duration;

use anyhow::Result;
use crossbeam_channel::{unbounded, Receiver};
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use notify_debouncer_mini::notify::{Config, RecommendedWatcher, RecursiveMode};
use notify_debouncer_mini::{
    new_debouncer_opt, Config as DebounceConfig, DebounceEventResult, DebouncedEvent, Debouncer,
};

/// Watch file system paths for changes.
///
/// Events are debounced to avoid duplicate notifications when files are rapidly
/// updated by editors. The watcher explicitly avoids following symlinks using
/// [`Config::with_follow_symlinks(false)`] to prevent duplicate events from files
/// that are referenced by multiple paths. A `vendor/` directory is ignored by
/// default and won't generate events.
pub struct FileWatcher {
    #[allow(dead_code)]
    debouncer: Debouncer<RecommendedWatcher>,
    rx: Receiver<DebounceEventResult>,
    ignore: Gitignore,
}

impl FileWatcher {
    /// Create a new watcher for the provided paths.
    pub fn new(paths: &[PathBuf]) -> Result<Self> {
        let (tx, rx) = unbounded();
        let config = DebounceConfig::default()
            .with_timeout(Duration::from_millis(200))
            .with_notify_config(Config::default().with_follow_symlinks(false));
        let mut debouncer = new_debouncer_opt(config, tx)?;

        for path in paths {
            debouncer.watcher().watch(path, RecursiveMode::Recursive)?;
        }

        let mut builder = GitignoreBuilder::new("");
        builder.add_line(None, "vendor/")?;
        let ignore = builder.build()?;

        Ok(FileWatcher {
            debouncer,
            rx,
            ignore,
        })
    }

    /// Wait for the next debounced change event.
    ///
    /// Returns a list of paths that were modified and not ignored. Paths are
    /// deduplicated before being returned.
    pub fn next(&self) -> Option<Vec<PathBuf>> {
        while let Ok(res) = self.rx.recv() {
            if let Ok(events) = res {
                let mut paths = Vec::new();
                for event in events.iter() {
                    collect_paths(event, &self.ignore, &mut paths);
                }
                if !paths.is_empty() {
                    paths.sort();
                    paths.dedup();
                    return Some(paths);
                }
            }
        }
        None
    }
}

fn collect_paths(event: &DebouncedEvent, ignore: &Gitignore, out: &mut Vec<PathBuf>) {
    let path = &event.path;
    if !ignore
        .matched_path_or_any_parents(path, path.is_dir())
        .is_ignore()
    {
        out.push(path.clone());
    }
}
