use std::path::Path;
use std::fs;

use anyhow::{anyhow, Result};
use tokio_stream::StreamExt;

use aider_llm::{ChatChunk, ModelProvider};

use crate::GitRepo;

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
    use aider_llm::mock::MockProvider;
    use tempfile::tempdir;
    use std::path::PathBuf;
    use git2::Repository;

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
}
