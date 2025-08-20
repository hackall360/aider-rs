use anyhow::Result;
use tokio_stream::StreamExt;

use aider_llm::{ChatChunk, ModelProvider};

use crate::GitRepo;

/// Generate a commit message using the given provider and staged diff.
///
/// The message follows conventional commit style. If the provider fails
/// or returns an empty response, a heuristic message is generated based on
/// the changed files. Optional co-author lines can be supplied via the
/// `AIDER_CO_AUTHORS` environment variable as a semicolon separated list.
pub async fn generate_commit_message(
    provider: Option<&dyn ModelProvider>,
    repo: &GitRepo,
) -> Result<String> {
    let status = repo.status(false)?;
    let files: Vec<String> = status
        .staged
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect();
    let diff = repo.diff_staged().unwrap_or_default();
    let summary: String = diff.lines().take(20).collect::<Vec<_>>().join("\n");

    let mut message = String::new();
    if let Some(p) = provider {
        let prompt = format!(
            "Generate a conventional commit message for the following changes.\nFiles:\n{}\nDiff:\n{}\nCommit message:",
            files.join("\n"),
            summary
        );
        let mut stream = p.chat(prompt);
        while let Some(chunk) = stream.next().await {
            if let ChatChunk::Token(tok) = chunk {
                message.push_str(&tok);
            }
        }
        message = message.trim().to_string();
    }

    if message.is_empty() {
        message = fallback_message(&files, &summary);
    }

    if let Ok(co_authors) = std::env::var("AIDER_CO_AUTHORS") {
        for author in co_authors
            .split(';')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
        {
            message.push_str(&format!("\nCo-authored-by: {}", author));
        }
    }

    Ok(message)
}

fn fallback_message(files: &[String], diff: &str) -> String {
    let prefix = if files.iter().any(|f| f.contains("test")) {
        "test"
    } else if files.iter().any(|f| f.ends_with(".md")) {
        "docs"
    } else if diff.to_lowercase().contains("fix") || diff.to_lowercase().contains("bug") {
        "fix"
    } else if files
        .iter()
        .all(|f| f.ends_with(".toml") || f.ends_with(".json") || f.ends_with(".yaml"))
    {
        "chore"
    } else {
        "feat"
    };

    if let Some(first) = files.first() {
        format!("{}: update {}", prefix, first)
    } else {
        format!("{}: update", prefix)
    }
}
