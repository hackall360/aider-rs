use anyhow::Result;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

pub mod command;
pub mod commit;
pub mod edit;
pub mod git;
pub mod runner;
pub mod session;
pub mod url;
pub mod voice;
pub mod watch;
pub use aider_llm::{mock::MockProvider, ModelProvider};
pub use command::Command;
pub use commit::generate_commit_message;
pub use edit::{apply_diff_edit, apply_whole_file_edit};
pub use git::{GitRepo, RepoStatus};
pub use runner::{apply_with_runner, CommandResult, JsRunner, RunOptions, Runner, RustRunner};
pub use session::{Mode, Session};
pub use voice::VoiceTranscriber;
pub use watch::FileWatcher;

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
