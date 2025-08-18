use assert_cmd::prelude::*;
use std::fs;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn prints_repomap_respecting_token_limit() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let file = dir.path().join("src.rs");
    fs::write(&file, "fn hello() {}\nfn world() {}\n")?;

    let mut cmd = Command::cargo_bin("aider-cli")?;
    cmd.current_dir(dir.path())
        .arg("--repomap")
        .arg("--map-tokens")
        .arg("5");
    let output = cmd.output()?;
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout)?;
    assert!(stdout.contains("hello"));
    assert!(!stdout.contains("world"));
    Ok(())
}
