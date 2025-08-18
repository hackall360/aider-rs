use anyhow::Result;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

pub mod session;
pub use session::Session;

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
