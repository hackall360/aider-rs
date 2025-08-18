use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
struct Args {}

fn main() -> Result<()> {
    let _ = Args::parse();
    aider_core::init_tracing()?;
    println!("aider cli placeholder");
    Ok(())
}
