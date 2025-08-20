use aider_core::{apply_with_runner, GitRepo, ModelProvider, RunOptions, RustRunner};
use aider_llm::{ChatChunk, Usage};
use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use tempfile::tempdir;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

struct SeqProvider {
    responses: Mutex<Vec<Vec<String>>>,
}

impl SeqProvider {
    fn new(responses: Vec<Vec<String>>) -> Self {
        Self {
            responses: Mutex::new(responses),
        }
    }
}

impl ModelProvider for SeqProvider {
    fn chat(&self, _prompt: String) -> ReceiverStream<ChatChunk> {
        let tokens = {
            let mut lock = self.responses.lock().unwrap();
            if lock.is_empty() {
                Vec::new()
            } else {
                lock.remove(0)
            }
        };
        let (tx, rx) = mpsc::channel(32);
        tokio::spawn(async move {
            for tok in tokens {
                let _ = tx.send(ChatChunk::Token(tok)).await;
            }
        });
        ReceiverStream::new(rx)
    }

    fn usage(&self) -> Usage {
        Usage::default()
    }
}

#[tokio::test]
async fn auto_fix_rust_runner() -> Result<()> {
    let dir = tempdir()?;
    fs::create_dir(dir.path().join("src"))?;
    fs::write(
        dir.path().join("Cargo.toml"),
        "[package]\nname=\"tmp\"\nversion=\"0.1.0\"\nedition=\"2021\"\n",
    )?;
    fs::write(
        dir.path().join("src/lib.rs"),
        "pub fn add(a:i32,b:i32)->i32{a+b}\n\n#[cfg(test)]\nmod tests{use super::*;#[test]fn it_adds(){assert_eq!(add(1,1),2);}}\n",
    )?;
    git2::Repository::init(dir.path())?;
    let git = GitRepo::open(dir.path())?;
    git.stage("Cargo.toml")?;
    git.stage(PathBuf::from("src/lib.rs"))?;
    git.commit("init")?;

    let bug_patch =
        "```diff\n@@\n-pub fn add(a:i32,b:i32)->i32{a+b}\n+pub fn add(a:i32,b:i32)->i32{a-b}\n```";
    let fix_patch =
        "```diff\n@@\n-pub fn add(a:i32,b:i32)->i32{a-b}\n+pub fn add(a:i32,b:i32)->i32{a+b}\n```";
    let provider = SeqProvider::new(vec![vec![bug_patch.into()], vec![fix_patch.into()]]);
    let runner = RustRunner::new(dir.path());
    let file_rel = PathBuf::from("src/lib.rs");
    let opts = RunOptions {
        no_lint: true,
        no_test: false,
        max_fix_attempts: 1,
    };
    let results = apply_with_runner(
        &provider,
        &git,
        &runner,
        &file_rel,
        "introduce bug",
        Some("bug"),
        true,
        opts,
    )
    .await?;
    assert!(results.iter().all(|r| r.status == 0));
    Ok(())
}
