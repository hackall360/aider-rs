mod analytics;
mod args;
mod config;
mod history;
mod prompts;
mod repo;
mod repomap;
mod resources;
mod watch;

use anyhow::Result;
use clap::Parser;
use tracing::info;
use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter};

use args::CliArgs;
use config::Config;

fn run() -> Result<()> {
    // Parse command line options using `clap`.
    let args = CliArgs::parse();

    // Load configuration and optionally save it back.
    let mut config = Config::load(args.config.as_deref())?;
    if args.verbose {
        config.verbose = true;
    }

    // Set up structured logging.
    let level =
        config
            .log_level
            .as_deref()
            .unwrap_or(if config.verbose { "debug" } else { "info" });
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(level))
        .with_span_events(FmtSpan::CLOSE)
        .json()
        .init();

    info!(?config, "configuration loaded");

    if args.save_config {
        config.save(args.config.as_deref())?;
    }
    if args.print_config {
        println!("{}", serde_yaml::to_string(&config)?);
        return Ok(());
    }

    // Enumerate repository files and parse the first one with tree-sitter.
    let repo = repo::Repo::new(std::env::current_dir()?);
    let mut repomap = repomap::RepoMap::new()?;
    analyze_repo(&repo, &mut repomap)?;

    // Set up analytics and history persistence.
    let data_dir = Config::data_dir();
    let mut analytics = analytics::Analytics::new(data_dir.join("analytics.yaml"));
    analytics.record("start");
    let mut history = history::History::new(data_dir.join("history.yaml"));
    history.add("run".to_string());

    // Demonstrate prompt template rendering from resources/templates.
    let prompts = prompts::Prompts::new()?;
    let mut ctx = tera::Context::new();
    ctx.insert("name", "world");
    let rendered = prompts.render("welcome.tera", &ctx)?;
    info!(%rendered, "rendered template");

    // Demonstrate loading resources in multiple formats.
    let _meta = resources::load_json("resources/model-metadata.json")?;
    let _settings = resources::load_yaml("resources/model-settings.yml")?;

    // Start file watching in the background.
    let _watcher = watch::watch(&std::env::current_dir()?)?;

    Ok(())
}

fn main() -> Result<()> {
    run()
}

#[tracing::instrument(skip(repo, repomap))]
fn analyze_repo(repo: &repo::Repo, repomap: &mut repomap::RepoMap) -> Result<()> {
    let files = repo.files();
    if let Some(first) = files.first() {
        let tree = repomap.parse_file(first)?;
        info!(file = ?first, nodes = tree.root_node().child_count(), "parsed file");
    }
    Ok(())
}
