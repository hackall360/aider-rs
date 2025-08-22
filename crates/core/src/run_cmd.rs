use anyhow::Result;
use atty::Stream;
use std::path::Path;
use tokio::process::Command;

/// Run a shell command asynchronously.
///
/// If stdin and stdout are attached to a TTY, the child process inherits the
/// terminal so the user can interact with it directly. In this case the output
/// is not captured and an empty string is returned.
///
/// When not connected to a TTY the output of the command is captured and
/// returned to the caller.
pub async fn run_cmd(command: &str, cwd: Option<&Path>) -> Result<(i32, String)> {
    let mut cmd = Command::new("sh");
    cmd.arg("-c").arg(command);
    if let Some(dir) = cwd {
        cmd.current_dir(dir);
    }

    // Interactive when both stdin and stdout are TTYs.
    if atty::is(Stream::Stdin) && atty::is(Stream::Stdout) {
        let status = cmd
            .stdin(std::process::Stdio::inherit())
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .status()
            .await?;
        Ok((status.code().unwrap_or_default(), String::new()))
    } else {
        let output = cmd
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .output()
            .await?;
        let mut text = String::from_utf8_lossy(&output.stdout).to_string();
        text.push_str(&String::from_utf8_lossy(&output.stderr));
        Ok((output.status.code().unwrap_or_default(), text))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_run_cmd_echo() {
        let (status, output) = run_cmd("echo hello", None).await.unwrap();
        assert_eq!(status, 0);
        assert_eq!(output.trim(), "hello");
    }
}
