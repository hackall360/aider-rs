use std::fs;

use aider_core::GitRepo;
use git2::Repository;
use tempfile::tempdir;

#[test]
fn detects_file_change_with_git() {
    let dir = tempdir().unwrap();
    Repository::init(dir.path()).unwrap();
    let git = GitRepo::open(dir.path()).unwrap();
    let file = dir.path().join("file.txt");
    fs::write(&file, "old\n").unwrap();
    git.stage("file.txt").unwrap();
    git.commit("init").unwrap();
    fs::write(&file, "new\n").unwrap();
    let diff = git.diff_unstaged().unwrap();
    assert!(diff.contains("-old"));
    assert!(diff.contains("+new"));
}
