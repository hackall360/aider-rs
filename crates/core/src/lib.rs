use anyhow::Result;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

pub mod command;
pub mod commit;
pub mod copypaste;
pub mod diffs;
pub mod edit;
pub mod git;
pub mod runner;
pub mod session;
pub mod url;
pub mod voice;
pub mod watch;
pub mod watch_prompts;
pub use aider_llm::{mock::MockProvider, ModelProvider};
pub use command::Command;
pub use commit::generate_commit_message;
pub use copypaste::ClipboardWatcher;
pub use diffs::{diff_partial_update, unified_diff};
pub use edit::{apply_diff_edit, apply_whole_file_edit};
pub use git::{GitRepo, RepoStatus};
pub use runner::{apply_with_runner, CommandResult, JsRunner, RunOptions, Runner, RustRunner};
pub use session::{Mode, Session};
pub use voice::VoiceTranscriber;
pub use watch::FileWatcher;
pub use watch_prompts::{watch_ask_prompt, watch_code_prompt};

pub fn init_tracing() -> Result<()> {
    let subscriber = FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    info!("tracing initialized");
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("example error")]
    Example,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_works() {
        init_tracing().unwrap();
    }
}
