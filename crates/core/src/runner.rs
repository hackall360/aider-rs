use crate::run_cmd;
use anyhow::Result;
use std::path::{Path, PathBuf};

/// Result of running a command.
#[derive(Debug, Default, Clone)]
pub struct CommandResult {
    pub command: String,
    pub output: String,
    pub status: i32,
}

/// Generic interface for language-specific linters and test runners.
#[async_trait::async_trait]
pub trait Runner {
    /// Name of the runner implementation.
    fn name(&self) -> &str;
    /// Run lint and/or tests and return command results.
    async fn run(&self, no_lint: bool, no_test: bool) -> Result<Vec<CommandResult>>;
}

/// Runner implementation for Rust projects using Cargo.
pub struct RustRunner {
    root: PathBuf,
}

impl RustRunner {
    pub fn new(root: impl AsRef<Path>) -> Self {
        Self {
            root: root.as_ref().to_path_buf(),
        }
    }
}

#[async_trait::async_trait]
impl Runner for RustRunner {
    fn name(&self) -> &str {
        "rust"
    }

    async fn run(&self, no_lint: bool, no_test: bool) -> Result<Vec<CommandResult>> {
        let mut results = Vec::new();
        if !no_lint {
            let cmd = "cargo clippy";
            let (status, output) = run_cmd(cmd, Some(&self.root)).await?;
            results.push(CommandResult {
                command: cmd.into(),
                output,
                status,
            });
        }
        if !no_test {
            let cmd = "cargo test";
            let (status, output) = run_cmd(cmd, Some(&self.root)).await?;
            results.push(CommandResult {
                command: cmd.into(),
                output,
                status,
            });
        }
        Ok(results)
    }
}

/// Runner implementation for JavaScript projects using npm.
pub struct JsRunner {
    root: PathBuf,
}

impl JsRunner {
    pub fn new(root: impl AsRef<Path>) -> Self {
        Self {
            root: root.as_ref().to_path_buf(),
        }
    }
}

#[async_trait::async_trait]
impl Runner for JsRunner {
    fn name(&self) -> &str {
        "javascript"
    }

    async fn run(&self, no_lint: bool, no_test: bool) -> Result<Vec<CommandResult>> {
        let mut results = Vec::new();
        if !no_test {
            let cmd = "npm test";
            let (status, output) = run_cmd(cmd, Some(&self.root)).await?;
            results.push(CommandResult {
                command: cmd.into(),
                output,
                status,
            });
        }
        if !no_lint {
            let cmd = "npm run lint";
            let (status, output) = run_cmd(cmd, Some(&self.root)).await?;
            results.push(CommandResult {
                command: cmd.into(),
                output,
                status,
            });
        }
        Ok(results)
    }
}

/// Options controlling how runners are executed.
#[derive(Debug, Clone, Copy)]
pub struct RunOptions {
    pub no_lint: bool,
    pub no_test: bool,
    pub max_fix_attempts: u32,
}

impl Default for RunOptions {
    fn default() -> Self {
        Self {
            no_lint: false,
            no_test: false,
            max_fix_attempts: 1,
        }
    }
}

/// Summarize output by returning the first `n` lines and the exit status.
pub fn summarize_output(text: &str, n: usize, status: i32) -> String {
    let lines: Vec<&str> = text.lines().take(n).collect();
    let mut summary = lines.join("\n");
    summary.push_str(&format!("\n(exit status: {status})"));
    summary
}

/// Apply a change and run runners, attempting to fix failures once.
pub async fn apply_with_runner(
    provider: &dyn crate::ModelProvider,
    repo: &crate::GitRepo,
    runner: &dyn Runner,
    file: &Path,
    change_request: &str,
    commit_message: Option<&str>,
    autocommit: bool,
    opts: RunOptions,
) -> Result<Vec<CommandResult>> {
    crate::apply_diff_edit(
        provider,
        repo,
        file,
        change_request,
        commit_message,
        autocommit,
    )
    .await?;
    let mut attempts = 0;
    loop {
        let results = runner.run(opts.no_lint, opts.no_test).await?;
        let failed = results.iter().find(|r| r.status != 0);
        if failed.is_none() {
            return Ok(results);
        }
        attempts += 1;
        if attempts > opts.max_fix_attempts {
            return Ok(results);
        }
        let failure = failed.unwrap();
        let summary = summarize_output(&failure.output, 20, failure.status);
        let fix_request = format!(
            "The following run of `{}` failed:\n{}\nPlease fix the code in {} so that the run succeeds.",
            failure.command,
            summary,
            file.display(),
        );
        crate::apply_diff_edit(
            provider,
            repo,
            file,
            &fix_request,
            Some("auto-fix"),
            autocommit,
        )
        .await?;
    }
}
