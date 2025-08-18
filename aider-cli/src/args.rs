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

    /// Print the merged configuration and exit.
    #[arg(long)]
    pub print_config: bool,

    /// Print a repository map built with tree-sitter and exit.
    #[arg(long)]
    pub repomap: bool,

    /// Maximum number of tokens to include in the repository map.
    #[arg(long, default_value_t = 1000)]
    pub map_tokens: usize,
}
