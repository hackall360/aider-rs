use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use tokio::task;
use crossbeam_channel::unbounded;

use crate::repo::Repo;

/// Watch the repository for a single change and return the relative paths of
/// modified files. The heavy lifting is performed in a blocking thread so the
/// async runtime isn't held by non-Send types.
pub async fn watch_once(repo: Repo) -> Result<Vec<String>, anyhow::Error> {
    let root = repo.root;
    let ignore = repo.ignore;
    let paths = task::spawn_blocking(move || {
        let (tx, rx) = unbounded();
        let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
        watcher.watch(&root, RecursiveMode::Recursive)?;
        let event = rx.recv()??;
        drop(watcher);
        let paths = event
            .paths
            .into_iter()
            .filter(|p| !ignore.matched_path_or_any_parents(p, p.is_dir()).is_ignore())
            .filter_map(|p| {
                p.strip_prefix(&root)
                    .ok()
                    .map(|s| s.to_string_lossy().to_string())
            })
            .collect();
        Ok::<_, notify::Error>(paths)
    })
    .await??;
    Ok(paths)
}

