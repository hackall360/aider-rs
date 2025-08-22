use anyhow::Result;
use std::path::PathBuf;
use std::process::Stdio;
use tokio::fs;
use tokio::process::Command;

/// Write data to a temporary file and return its path.
pub async fn write_temp_file(data: &str, suffix: Option<&str>) -> Result<PathBuf> {
    let mut builder = tempfile::Builder::new();
    let owned_suffix = suffix.map(|s| format!(".{s}"));
    if let Some(ref suf) = owned_suffix {
        builder.suffix(suf);
    }
    let file = builder.tempfile()?;
    let (_f, path) = file.keep()?;
    fs::write(&path, data).await?;
    Ok(path)
}

/// Open the system editor with the provided input and return the edited text.
pub async fn pipe_editor(
    input: &str,
    suffix: Option<&str>,
    editor: Option<&str>,
) -> Result<String> {
    let path = write_temp_file(input, suffix).await?;
    let editor_cmd = editor
        .map(|s| s.to_string())
        .unwrap_or_else(|| std::env::var("EDITOR").unwrap_or_else(|_| "vi".into()));
    let mut cmd = Command::new("sh");
    cmd.arg("-c")
        .arg(format!("{editor_cmd} {}", path.display()));
    cmd.stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .await?;
    let output = fs::read_to_string(&path).await?;
    let _ = fs::remove_file(&path).await;
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    // We can't easily test spawning an external editor in CI, but we can ensure
    // the temp file is created and returned.
    #[tokio::test]
    async fn test_write_temp_file() {
        let path = write_temp_file("hi", Some("txt")).await.unwrap();
        assert!(path.exists());
        fs::remove_file(&path).await.unwrap();
    }
}
