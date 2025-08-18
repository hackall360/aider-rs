use anyhow::Result;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

pub mod git;
pub mod model;
pub mod session;
pub mod watch;
pub use git::{GitRepo, RepoStatus};
pub use model::{EchoModel, Model};
pub use session::Session;
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
