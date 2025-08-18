use assert_cmd::prelude::*;
use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::tempdir;

fn run_map(dir: &Path, tokens: &str) -> Vec<String> {
    let mut cmd = Command::cargo_bin("aider-cli").unwrap();
    cmd.current_dir(dir)
        .arg("--repomap")
        .arg("--map-tokens")
        .arg(tokens);
    let output = cmd.output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8(output.stdout).unwrap();
    let mut files = Vec::new();
    for line in stdout.lines() {
        if line.starts_with('{') {
            continue;
        }
        if let Some((path, _)) = line.split_once(':') {
            if let Some(name) = std::path::Path::new(path).file_name().and_then(|s| s.to_str()) {
                if !files.contains(&name.to_string()) {
                    files.push(name.to_string());
                }
            }
        }
    }
    files
}

#[test]
fn selection_changes_with_budget() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("a.rs"), "use crate::b;\nuse crate::c;\nfn a() {}\n").unwrap();
    fs::write(dir.path().join("b.rs"), "use crate::c;\nfn b() {}\n").unwrap();
    fs::write(dir.path().join("c.rs"), "fn c() {}\n").unwrap();

    let files = run_map(dir.path(), "3");
    assert_eq!(files, vec!["c.rs"]);

    let files = run_map(dir.path(), "8");
    assert_eq!(files, vec!["c.rs", "b.rs"]);

    let files = run_map(dir.path(), "20");
    assert_eq!(files, vec!["c.rs", "b.rs", "a.rs"]);
}

#[test]
fn selection_deterministic_with_large_budget() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("a.rs"), "use crate::b;\nuse crate::c;\nfn a() {}\n").unwrap();
    fs::write(dir.path().join("b.rs"), "use crate::c;\nfn b() {}\n").unwrap();
    fs::write(dir.path().join("c.rs"), "fn c() {}\n").unwrap();

    let files1 = run_map(dir.path(), "1000");
    let files2 = run_map(dir.path(), "1000");
    assert_eq!(files1, files2);
}
