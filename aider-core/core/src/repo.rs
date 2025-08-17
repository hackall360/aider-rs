use std::path::{Path, PathBuf};

use gix::{self, Repository};
use ignore::{gitignore::GitignoreBuilder, WalkBuilder};

/// Simple Git repository wrapper providing access to tracked files
/// while respecting ignore rules.
pub struct Repo {
    pub root: PathBuf,
    repo: Repository,
    pub(crate) ignore: ignore::gitignore::Gitignore,
}

impl Repo {
    /// Open an existing git repository rooted at `path`.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, gix::open::Error> {
        let repo = gix::open(path.as_ref())?;
        let workdir = repo
            .work_dir()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| path.as_ref().to_path_buf());

        let mut builder = GitignoreBuilder::new(&workdir);
        builder.add(workdir.join(".gitignore"));
        let ignore = builder.build().unwrap_or_else(|_| GitignoreBuilder::new(".").build().unwrap());

        Ok(Self {
            root: workdir,
            repo,
            ignore,
        })
    }

    /// Return all non-ignored files within the repository.
    pub fn files(&self) -> Vec<PathBuf> {
        WalkBuilder::new(&self.root)
            .standard_filters(true)
            .build()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().map(|ft| ft.is_file()).unwrap_or(false))
            .map(|e| e.into_path())
            .collect()
    }

    /// Return the current HEAD commit id as a hex string.
    pub fn head(&self) -> Result<String, gix::reference::find::existing::Error> {
        let id = self
            .repo
            .head()?
            .id()
            .expect("HEAD to point to object")
            .detach();
        Ok(id.to_hex().to_string())
    }

    /// Check if a path is ignored by the repository's ignore rules.
    pub fn is_ignored(&self, path: &Path) -> bool {
        self.ignore
            .matched_path_or_any_parents(path, path.is_dir())
            .is_ignore()
    }
}

