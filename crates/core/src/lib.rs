use anyhow::Result;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

pub mod command;
pub mod commit;
pub mod copypaste;
pub mod diffs;
pub mod edit;
pub mod editor;
pub mod git;
pub mod io;
pub mod help;
pub mod run_cmd;
pub mod runner;
pub mod session;
pub mod url;
pub mod utils;
pub mod onboarding;
pub mod voice;
pub mod watch;
pub mod watch_prompts;
pub mod models;
pub use aider_llm::{mock::MockProvider, ModelProvider};
pub use command::Command;
pub use commit::generate_commit_message;
pub use copypaste::ClipboardWatcher;
pub use diffs::{diff_partial_update, unified_diff};
pub use edit::{apply_diff_edit, apply_whole_file_edit};
pub use editor::{pipe_editor, write_temp_file};
pub use git::{GitRepo, RepoStatus};
pub use io::{ensure_hash_prefix, is_tty};
pub use help::{fname_to_url, Help, Doc, Retriever};
pub use models::{ModelInfo, ModelInfoManager};
pub use run_cmd::run_cmd;
pub use runner::{apply_with_runner, CommandResult, JsRunner, RunOptions, Runner, RustRunner};
pub use session::{Mode, Session};
pub use utils::{is_image_file, safe_abs_path};
pub use onboarding::{check_openrouter_tier, try_to_select_default_model};
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
