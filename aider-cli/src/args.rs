use std::path::PathBuf;

use clap::Parser;

/// Command line arguments mapping the original Python `args.py` module.
#[derive(Debug, Parser)]
#[command(name = "aider-cli")]
pub struct CliArgs {
    /// Optional path to a configuration file.
    #[arg(short, long)]
    pub config: Option<PathBuf>,

    /// Enable verbose output.
    #[arg(short, long)]
    pub verbose: bool,

    /// Persist the current settings to the configuration file.
    #[arg(long)]
    pub save_config: bool,
}
