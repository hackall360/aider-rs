use std::path::{Path, PathBuf};

use ignore::WalkBuilder;

/// Repository helper mirroring the Python `repo.py` module.
pub struct Repo {
    root: PathBuf,
}

impl Repo {
    /// Create a repository wrapper rooted at the given path.
    pub fn new(root: impl AsRef<Path>) -> Self {
        Self {
            root: root.as_ref().to_path_buf(),
        }
    }

    /// Return a list of all files respecting `.gitignore` rules.
    pub fn files(&self) -> Vec<PathBuf> {
        WalkBuilder::new(&self.root)
            .standard_filters(true)
            .build()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().map(|ft| ft.is_file()).unwrap_or(false))
            .map(|e| e.into_path())
            .collect()
    }
}
