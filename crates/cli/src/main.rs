use anyhow::Result;
use clap::Parser;

/// Command line interface for the aider core library.
///
/// The real application will provide an interactive coding session. For now
/// we parse a few options and forward them to a stub session so that the
/// binary can be exercised.
#[derive(Parser, Debug)]
#[command(
    name = "aider",
    version,
    about = "AI pair programmer",
    after_help = "EXAMPLES:\n  aider --model gpt-4\n  aider --dry-run \"Refactor main.rs\""
)]
struct Args {
    /// LLM model to use
    #[arg(long, default_value = "gpt-4")]
    model: String,

    /// OpenAI API key (also read from OPENAI_API_KEY env variable)
    #[arg(long, env = "OPENAI_API_KEY")]
    openai_api_key: Option<String>,

    /// Don't write any changes, just show what would happen
    #[arg(long)]
    dry_run: bool,

    /// Optional prompt to start the session
    #[arg()]
    prompt: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    aider_core::init_tracing()?;
    let prompt = if args.prompt.is_empty() {
        None
    } else {
        Some(args.prompt.join(" "))
    };

    let mut session =
        aider_core::Session::new(args.model, prompt, args.openai_api_key, args.dry_run);
    session.run().await?;
    Ok(())
}
