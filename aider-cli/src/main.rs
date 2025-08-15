mod analytics;
mod args;
mod history;
mod prompts;
mod repo;
mod repomap;
mod resources;
mod settings;
mod watch;

use anyhow::Result;
use clap::Parser;

use args::CliArgs;
use settings::Settings;

fn main() -> Result<()> {
    // Parse command line options using `clap`.
    let args = CliArgs::parse();

    // Load persisted settings if present and optionally save them back.
    let settings = Settings::load_or_default(args.config.as_deref())?;
    if args.save_config {
        settings.save(args.config.as_deref())?;
    }

    if args.verbose || settings.verbose {
        println!("Verbose mode enabled");
    }

    // Enumerate repository files and parse the first one with tree-sitter.
    let repo = repo::Repo::new(std::env::current_dir()?);
    let files = repo.files();
    let mut repomap = repomap::RepoMap::new()?;
    if let Some(first) = files.first() {
        let tree = repomap.parse_file(first)?;
        println!(
            "Parsed {:?} with {} top-level nodes",
            first,
            tree.root_node().child_count()
        );
    }

    // Set up analytics and history persistence.
    let data_dir = Settings::data_dir();
    let mut analytics = analytics::Analytics::new(data_dir.join("analytics.yaml"));
    analytics.record("start");
    let mut history = history::History::new(data_dir.join("history.yaml"));
    history.add("run".to_string());

    // Demonstrate prompt template rendering.
    let mut prompts = prompts::Prompts::default();
    let mut ctx = tera::Context::new();
    ctx.insert("name", "world");
    let rendered = prompts.render_str("Hello {{name}}!", &ctx)?;
    if args.verbose {
        println!("{}", rendered);
    }

    // Demonstrate loading resources in multiple formats.
    let _meta = resources::load_json("resources/model-metadata.json")?;
    let _settings = resources::load_yaml("resources/model-settings.yml")?;
    let _prompt = resources::load_prompt("resources/prompts/welcome.mustache")?;

    // Start file watching in the background.
    let _watcher = watch::watch(&std::env::current_dir()?)?;

    Ok(())
}
