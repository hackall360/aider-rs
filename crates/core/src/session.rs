use anyhow::Result;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt};
use tokio::signal;
use tokio_stream::StreamExt;

use crate::model::Model;

#[derive(Default)]
enum ChatMode {
    /// Basic chat mode that simply streams model output.
    #[default]
    Echo,
}

/// Core interactive session managing state and user interaction.
pub struct Session {
    model_name: String,
    prompt: Option<String>,
    api_key: Option<String>,
    dry_run: bool,
    history: Vec<String>,
    model: Box<dyn Model>,
    chat_mode: ChatMode,
}

impl Session {
    /// Create a new session with the chosen options.
    pub fn new(
        model_name: String,
        prompt: Option<String>,
        api_key: Option<String>,
        dry_run: bool,
    ) -> Self {
        let model: Box<dyn Model> = Box::new(crate::model::EchoModel::default());
        Self {
            model_name,
            prompt,
            api_key,
            dry_run,
            history: Vec::new(),
            model,
            chat_mode: ChatMode::default(),
        }
    }

    /// Run the interactive session.
    pub async fn run(&mut self) -> Result<()> {
        println!("Starting aider session with model: {}", self.model_name);
        match self.api_key {
            Some(_) => println!("API key support is not implemented; key ignored."),
            None => println!("No API key provided; network features are disabled."),
        }
        if self.dry_run {
            println!("Dry-run mode selected. No changes will be written.");
        }

        if let Some(msg) = self.prompt.take() {
            self.handle_message(msg).await?;
        }

        let stdin = io::stdin();
        let mut reader = io::BufReader::new(stdin);
        let mut stdout = io::stdout();
        let mut line = String::new();

        loop {
            stdout.write_all(b"> ").await?;
            stdout.flush().await?;
            line.clear();
            let bytes = reader.read_line(&mut line).await?;
            if bytes == 0 {
                break;
            }
            let input = line.trim().to_string();
            if input.is_empty() {
                continue;
            }
            if input == "/exit" || input == "/quit" {
                break;
            }
            self.handle_message(input).await?;
        }
        Ok(())
    }

    async fn handle_message(&mut self, message: String) -> Result<()> {
        self.history.push(message.clone());
        match self.chat_mode {
            ChatMode::Echo => self.stream_reply(message).await?,
        }
        Ok(())
    }

    async fn stream_reply(&self, message: String) -> Result<()> {
        let mut stream = self.model.chat(message);
        let mut stdout = io::stdout();
        let ctrl_c = signal::ctrl_c();
        tokio::pin!(ctrl_c);

        loop {
            tokio::select! {
                token = stream.next() => {
                    match token {
                        Some(t) => {
                            stdout.write_all(t.as_bytes()).await?;
                            stdout.flush().await?;
                        }
                        None => {
                            stdout.write_all(b"\n").await?;
                            stdout.flush().await?;
                            break;
                        }
                    }
                }
                _ = &mut ctrl_c => {
                    stdout.write_all(b"\nGeneration cancelled.\n").await?;
                    stdout.flush().await?;
                    break;
                }
            }
        }
        Ok(())
    }
}
