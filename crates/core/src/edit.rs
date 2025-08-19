use std::path::Path;
use std::fs;

use diffy::{apply, Patch};

use anyhow::{anyhow, Result};
use tokio_stream::StreamExt;

use aider_llm::{ChatChunk, ModelProvider};

use crate::GitRepo;

/// Apply a diff-based edit by asking the model to return a unified diff.
///
/// The model should return a patch inside triple backticks. The patch is
/// validated and applied to the target file. If the patch cannot be parsed or
/// applied cleanly, this function falls back to a whole-file edit.
pub async fn apply_diff_edit(
    provider: &dyn ModelProvider,
    repo: &GitRepo,
    file: &Path,
    change_request: &str,
    commit_message: &str,
) -> Result<String> {
    let prompt = format!(
        "Apply the following changes to `{}`:\n{}\nReturn a unified diff inside triple backticks.",
        file.display(),
        change_request
    );

    let mut stream = provider.chat(prompt);
    let mut output = String::new();
    while let Some(chunk) = stream.next().await {
        if let ChatChunk::Token(tok) = chunk {
            output.push_str(&tok);
        }
    }

    let diff_text = match extract_fenced_code(&output) {
        Ok(t) => t,
        Err(_) => {
            return apply_whole_file_edit(provider, repo, file, change_request, commit_message)
                .await;
        }
    };

    let patch = match Patch::from_str(&diff_text) {
        Ok(p) => p,
        Err(_) => {
            return apply_whole_file_edit(provider, repo, file, change_request, commit_message)
                .await;
        }
    };

    let full_path = repo.root().join(file);
    let original = fs::read_to_string(&full_path)?;
    let updated = match apply(&original, &patch) {
        Ok(u) => u,
        Err(_) => {
            return apply_whole_file_edit(provider, repo, file, change_request, commit_message)
                .await;
        }
    };

    fs::write(&full_path, updated)?;
    let diff = repo.diff_unstaged()?;
    repo.stage(file)?;
    repo.commit(commit_message)?;
    Ok(diff)
}

/// Apply a whole-file edit by asking the model to rewrite the entire file.
///
/// The model is prompted to return the full file contents inside a single
/// fenced code block. The contents are validated and written to disk. The
/// resulting diff is returned and a commit is created with the supplied
/// message.
pub async fn apply_whole_file_edit(
    provider: &dyn ModelProvider,
    repo: &GitRepo,
    file: &Path,
    change_request: &str,
    commit_message: &str,
) -> Result<String> {
    let prompt = format!(
        "Rewrite the file `{}` to satisfy this request:\n{}\nReturn the full contents of the file inside triple backticks.",
        file.display(),
        change_request
    );
    let mut stream = provider.chat(prompt);
    let mut output = String::new();
    while let Some(chunk) = stream.next().await {
        if let ChatChunk::Token(tok) = chunk {
            output.push_str(&tok);
        }
    }
    let contents = extract_fenced_code(&output)?;
    let full_path = repo.root().join(file);
    fs::write(&full_path, contents)?;
    let diff = repo.diff_unstaged()?;
    repo.stage(file)?;
    repo.commit(commit_message)?;
    Ok(diff)
}

fn extract_fenced_code(text: &str) -> Result<String> {
    let start = text
        .find("```")
        .ok_or_else(|| anyhow!("missing opening code fence"))?;
    let after_start = &text[start + 3..];
    let newline = after_start
        .find('\n')
        .ok_or_else(|| anyhow!("missing newline after fence"))?;
    let after_lang = &after_start[newline + 1..];
    let end = after_lang
        .find("```")
        .ok_or_else(|| anyhow!("missing closing code fence"))?;
    Ok(after_lang[..end].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use aider_llm::{mock::MockProvider, ChatChunk, ModelProvider, Usage};
    use tempfile::tempdir;
    use std::path::PathBuf;
    use git2::Repository;
    use tokio::sync::mpsc;
    use tokio_stream::wrappers::ReceiverStream;
    use std::sync::Mutex;

    #[tokio::test]
    async fn whole_file_edit_commits_changes() -> Result<()> {
        let dir = tempdir()?;
        Repository::init(dir.path())?;
        let git = GitRepo::open(dir.path())?;
        let file_rel = PathBuf::from("file.txt");
        let file_path = dir.path().join(&file_rel);
        fs::write(&file_path, "old\n")?;
        git.stage(&file_rel)?;
        git.commit("init")?;

        let response = "```text\nnew\n```";
        let provider = MockProvider::new_with_tokens(vec![response.into()]);
        let diff = apply_whole_file_edit(
            &provider,
            &git,
            &file_rel,
            "replace contents",
            "update file",
        )
        .await?;

        let new_contents = fs::read_to_string(&file_path)?;
        assert_eq!(new_contents, "new\n");
        assert!(diff.contains("-old"));
        assert!(diff.contains("+new"));

        // verify commit message
        let repo = Repository::open(dir.path())?;
        let head = repo.head()?.peel_to_commit()?;
        assert_eq!(head.message().unwrap(), "update file");
        Ok(())
    }

    #[tokio::test]
    async fn diff_edit_commits_changes() -> Result<()> {
        let dir = tempdir()?;
        Repository::init(dir.path())?;
        let git = GitRepo::open(dir.path())?;
        let file_rel = PathBuf::from("file.txt");
        let file_path = dir.path().join(&file_rel);
        fs::write(&file_path, "old\n")?;
        git.stage(&file_rel)?;
        git.commit("init")?;

        let response = "```diff\n@@ -1,1 +1,1 @@\n-old\n+new\n```";
        let provider = MockProvider::new_with_tokens(vec![response.into()]);
        let diff = apply_diff_edit(
            &provider,
            &git,
            &file_rel,
            "replace contents",
            "update file",
        )
        .await?;

        let new_contents = fs::read_to_string(&file_path)?;
        assert_eq!(new_contents, "new\n");
        assert!(diff.contains("-old"));
        assert!(diff.contains("+new"));

        Ok(())
    }

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
    async fn diff_edit_falls_back_to_whole_file() -> Result<()> {
        let dir = tempdir()?;
        Repository::init(dir.path())?;
        let git = GitRepo::open(dir.path())?;
        let file_rel = PathBuf::from("file.txt");
        let file_path = dir.path().join(&file_rel);
        fs::write(&file_path, "old\n")?;
        git.stage(&file_rel)?;
        git.commit("init")?;

        let diff_response = "```diff\n@@ -1,1 +1,1 @@\n-bad\n+new\n```";
        let whole_response = "```text\nnew\n```";
        let provider = SeqProvider::new(vec![
            vec![diff_response.into()],
            vec![whole_response.into()],
        ]);

        let diff = apply_diff_edit(
            &provider,
            &git,
            &file_rel,
            "replace contents",
            "update file",
        )
        .await?;

        let new_contents = fs::read_to_string(&file_path)?;
        assert_eq!(new_contents, "new\n");
        assert!(diff.contains("-old"));
        assert!(diff.contains("+new"));

        // verify commit message
        let repo = Repository::open(dir.path())?;
        let head = repo.head()?.peel_to_commit()?;
        assert_eq!(head.message().unwrap(), "update file");
        Ok(())
    }
}
