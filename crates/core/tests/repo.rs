use aider_core::GitRepo;
use anyhow::Result;
use std::fs;
use tempfile::tempdir;

#[test]
fn diffs_include_staged_and_unstaged_changes() -> Result<()> {
    // initialize empty repository
    let dir = tempdir()?;
    git2::Repository::init(dir.path())?;
    let git = GitRepo::open(dir.path())?;

    // create file and stage it
    let file = dir.path().join("foo.txt");
    fs::write(&file, "index\n")?;
    git.stage("foo.txt")?;

    // modify the working directory copy
    fs::write(&file, "workingdir\n")?;

    let staged = git.diff_staged()?;
    let unstaged = git.diff_unstaged()?;

    assert!(staged.contains("index"));
    assert!(unstaged.contains("workingdir"));
    Ok(())
}
