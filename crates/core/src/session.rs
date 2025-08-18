use anyhow::Result;

/// Placeholder for the core interactive session.
///
/// The real implementation will manage conversations with an LLM and
/// apply edits to the user's repository. For now we just record the
/// settings and emit helpful messages so the CLI can be exercised
/// without crashing.
#[derive(Debug)]
pub struct Session {
    model: String,
    prompt: Option<String>,
    api_key: Option<String>,
    dry_run: bool,
}

impl Session {
    /// Create a new session with the chosen options.
    pub fn new(
        model: String,
        prompt: Option<String>,
        api_key: Option<String>,
        dry_run: bool,
    ) -> Self {
        Self {
            model,
            prompt,
            api_key,
            dry_run,
        }
    }

    /// Run the session. Currently this only prints stub messages.
    pub fn run(&mut self) -> Result<()> {
        println!("Starting aider session with model: {}", self.model);
        if let Some(ref msg) = self.prompt {
            println!("Initial prompt: {msg}");
        }
        match self.api_key {
            Some(_) => println!("API key support is not implemented; key ignored."),
            None => println!("No API key provided; network features are disabled."),
        }
        if self.dry_run {
            println!("Dry-run mode selected. No changes will be written.");
        }
        println!("Interactive editing is not implemented yet.");
        Ok(())
    }
}
