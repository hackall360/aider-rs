use anyhow::{anyhow, Result};
use git2::{DiffFormat, Repository, Signature, StatusOptions};
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::path::{Path, PathBuf};

/// Represents the status of the repository.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct RepoStatus {
    pub staged: Vec<PathBuf>,
    pub unstaged: Vec<PathBuf>,
}

/// A simple Git repository manager built on top of `git2` and `ignore`.
pub struct GitRepo {
    repo: Repository,
    root: PathBuf,
    ignore: Gitignore,
}

impl GitRepo {
    /// Open and validate a repository, discovering the root directory.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let repo = Repository::discover(path)?;
        let root = repo
            .workdir()
            .ok_or_else(|| anyhow!("bare repositories are not supported"))?
            .to_path_buf();

        let mut builder = GitignoreBuilder::new(&root);
        // It's fine if there is no `.gitignore` file.
        let _ = builder.add(root.join(".gitignore"));
        let ignore = builder.build()?;

        Ok(Self { repo, root, ignore })
    }

    /// Return the repository root directory.
    pub fn root(&self) -> &Path {
        &self.root
    }

    fn is_ignored(&self, path: &Path) -> bool {
        self.ignore
            .matched_path_or_any_parents(path, path.is_dir())
            .is_ignore()
    }

    /// Return lists of staged and unstaged paths.
    pub fn status(&self, include_ignored: bool) -> Result<RepoStatus> {
        let mut opts = StatusOptions::new();
        opts.include_untracked(true)
            .recurse_untracked_dirs(true)
            .include_ignored(include_ignored);
        let statuses = self.repo.statuses(Some(&mut opts))?;
        let mut staged = Vec::new();
        let mut unstaged = Vec::new();

        for entry in statuses.iter() {
            let status = entry.status();
            let path = PathBuf::from(entry.path().unwrap_or_default());
            let full = self.root.join(&path);
            if !include_ignored && self.is_ignored(&full) {
                continue;
            }

            if status.is_index_new()
                || status.is_index_modified()
                || status.is_index_deleted()
                || status.is_index_renamed()
                || status.is_index_typechange()
            {
                staged.push(path.clone());
            }

            if status.is_wt_new()
                || status.is_wt_modified()
                || status.is_wt_deleted()
                || status.is_wt_renamed()
                || status.is_wt_typechange()
                || (include_ignored && status.is_ignored())
            {
                unstaged.push(path);
            }
        }

        Ok(RepoStatus { staged, unstaged })
    }

    /// Stage a single path.
    pub fn stage<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let mut index = self.repo.index()?;
        index.add_path(path.as_ref())?;
        index.write()?;
        Ok(())
    }

    /// Commit the current index with the given message.
    pub fn commit(&self, message: &str) -> Result<git2::Oid> {
        let mut index = self.repo.index()?;
        let tree_id = index.write_tree()?;
        let tree = self.repo.find_tree(tree_id)?;

        let signature = self
            .repo
            .signature()
            .or_else(|_| Signature::now("aider", "aider@example.com"))?;

        let parent = self
            .repo
            .head()
            .ok()
            .and_then(|h| h.target())
            .and_then(|oid| self.repo.find_commit(oid).ok());

        let oid = if let Some(parent) = parent {
            self.repo.commit(
                Some("HEAD"),
                &signature,
                &signature,
                message,
                &tree,
                &[&parent],
            )?
        } else {
            self.repo
                .commit(Some("HEAD"), &signature, &signature, message, &tree, &[])?
        };
        index.write()?;
        Ok(oid)
    }

    /// Return a unified diff of unstaged changes.
    pub fn diff_unstaged(&self) -> Result<String> {
        let index = self.repo.index()?;
        let diff = self.repo.diff_index_to_workdir(Some(&index), None)?;
        Self::diff_to_string(&diff)
    }

    /// Return a unified diff of staged changes.
    pub fn diff_staged(&self) -> Result<String> {
        let head = self.repo.head().ok();
        let tree = match head.and_then(|h| h.target()) {
            Some(oid) => Some(self.repo.find_commit(oid)?.tree()?),
            None => None,
        };
        let index = self.repo.index()?;
        let diff = self
            .repo
            .diff_tree_to_index(tree.as_ref(), Some(&index), None)?;
        Self::diff_to_string(&diff)
    }

    fn diff_to_string(diff: &git2::Diff) -> Result<String> {
        let mut out = Vec::new();
        diff.print(DiffFormat::Patch, |_delta, _hunk, line| {
            out.push(line.origin() as u8);
            out.extend_from_slice(line.content());
            true
        })?;
        Ok(String::from_utf8(out)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn stage_commit_and_diff() -> Result<()> {
        let dir = tempdir()?;
        Repository::init(dir.path())?;
        let git = GitRepo::open(dir.path())?;

        // create file
        fs::write(dir.path().join("file.txt"), "hello\n")?;
        let status = git.status(false)?;
        assert!(status.unstaged.contains(&PathBuf::from("file.txt")));

        git.stage("file.txt")?;
        let status = git.status(false)?;
        assert!(status.staged.contains(&PathBuf::from("file.txt")));

        git.commit("initial commit")?;
        let status = git.status(false)?;
        assert!(status.staged.is_empty() && status.unstaged.is_empty());

        // modify file
        fs::write(dir.path().join("file.txt"), "hello world\n")?;
        let diff = git.diff_unstaged()?;
        assert!(diff.contains("+hello world"));
        assert!(diff.contains("-hello"));
        Ok(())
    }

    #[test]
    fn respect_gitignore() -> Result<()> {
        let dir = tempdir()?;
        fs::write(dir.path().join(".gitignore"), "ignored.txt\n")?;
        Repository::init(dir.path())?;
        let git = GitRepo::open(dir.path())?;

        fs::write(dir.path().join("ignored.txt"), "data\n")?;
        let status = git.status(false)?;
        assert!(!status
            .unstaged
            .iter()
            .any(|p| p == Path::new("ignored.txt")));
        let status = git.status(true)?;
        assert!(status
            .unstaged
            .iter()
            .any(|p| p == Path::new("ignored.txt")));
        Ok(())
    }
}
