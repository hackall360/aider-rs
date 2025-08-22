use aider_core::Mode;
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

    /// Skip running linters
    #[arg(long)]
    no_lint: bool,

    /// Skip running tests
    #[arg(long)]
    no_test: bool,

    /// Do not automatically commit changes
    #[arg(long)]
    no_autocommit: bool,

    /// Maximum number of automatic fix attempts
    #[arg(long, default_value_t = 1)]
    max_fix_attempts: u32,

    /// Enable voice input using whisper
    #[arg(long)]
    voice: bool,

    /// Path to the whisper model
    #[arg(long, default_value = "models/ggml-tiny.en.bin")]
    voice_model: String,

    /// Voice activity detection threshold
    #[arg(long, default_value_t = 0.01)]
    vad_threshold: f32,

    /// Initial chat mode
    #[arg(long, default_value = "code")]
    mode: String,

    /// Optional shell command to run before starting
    #[arg(long)]
    run: Option<String>,

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

    let mode: Mode = args.mode.parse().unwrap_or_default();
    let voice = if args.voice {
        Some(aider_core::VoiceTranscriber::new(
            args.voice_model.into(),
            args.vad_threshold,
        ))
    } else {
        None
    };

    if let Some(cmd) = args.run {
        let (_status, output) = aider_core::run_cmd(&cmd, None).await?;
        if !output.is_empty() {
            print!("{}", output);
        }
    }
    let mut session = aider_core::Session::new(
        args.model,
        prompt,
        args.openai_api_key,
        args.dry_run,
        mode,
        args.no_lint,
        args.no_test,
        args.no_autocommit,
        args.max_fix_attempts,
        voice,
    );
    session.run().await?;
    Ok(())
}
