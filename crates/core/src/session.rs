use anyhow::Result;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt};
use tokio::signal;
use tokio_stream::StreamExt;
use tracing::debug;

use aider_llm::{mock::MockProvider, ChatChunk, ModelProvider};

/// Available chat modes that control prompting strategy.
#[derive(Debug, Clone)]
pub enum Mode {
    Code,
    Architect,
    Ask,
    Help,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Code
    }
}

impl Mode {
    fn system_prompt(&self) -> &'static str {
        match self {
            Mode::Code => "You are an AI coding assistant.",
            Mode::Architect => "You are an AI software architect.",
            Mode::Ask => "You are an AI assistant answering questions.",
            Mode::Help => "You are an AI assistant offering help.",
        }
    }
}

impl std::str::FromStr for Mode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "code" => Ok(Mode::Code),
            "architect" => Ok(Mode::Architect),
            "ask" => Ok(Mode::Ask),
            "help" => Ok(Mode::Help),
            _ => Err(format!("unknown mode: {s}")),
        }
    }
}

/// Core interactive session managing state and user interaction.
pub struct Session {
    model_name: String,
    prompt: Option<String>,
    api_key: Option<String>,
    dry_run: bool,
    history: Vec<String>,
    provider: Box<dyn ModelProvider>,
    mode: Mode,
}

impl Session {
    /// Create a new session with the chosen options.
    pub fn new(
        model_name: String,
        prompt: Option<String>,
        api_key: Option<String>,
        dry_run: bool,
        mode: Mode,
    ) -> Self {
        Self::with_provider(
            model_name,
            prompt,
            api_key,
            dry_run,
            mode,
            Box::new(MockProvider::default()),
        )
    }

    pub fn with_provider(
        model_name: String,
        prompt: Option<String>,
        api_key: Option<String>,
        dry_run: bool,
        mode: Mode,
        provider: Box<dyn ModelProvider>,
    ) -> Self {
        Self {
            model_name,
            prompt,
            api_key,
            dry_run,
            history: Vec::new(),
            provider,
            mode,
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
        if let Some(rest) = message.strip_prefix("/mode ") {
            match rest.parse::<Mode>() {
                Ok(mode) => {
                    self.mode = mode;
                    debug!("switched mode"=?self.mode, prompt=self.mode.system_prompt());
                }
                Err(err) => println!("{err}"),
            }
            return Ok(());
        }
        if let Some(rest) = message.strip_prefix("/ask ") {
            self.stream_reply_with_mode(Mode::Ask, rest.to_string())
                .await?;
            return Ok(());
        }
        if let Some(rest) = message.strip_prefix("/help ") {
            self.stream_reply_with_mode(Mode::Help, rest.to_string())
                .await?;
            return Ok(());
        }
        if let Some(rest) = message.strip_prefix("/architect ") {
            self.stream_reply_with_mode(Mode::Architect, rest.to_string())
                .await?;
            return Ok(());
        }
        if let Some(rest) = message.strip_prefix("/code ") {
            self.stream_reply_with_mode(Mode::Code, rest.to_string())
                .await?;
            return Ok(());
        }
        self.history.push(message.clone());
        self.stream_reply_with_mode(self.mode.clone(), message)
            .await?;
        Ok(())
    }

    async fn stream_reply_with_mode(&self, mode: Mode, message: String) -> Result<()> {
        let system_prompt = mode.system_prompt();
        debug!(mode=?mode, prompt=system_prompt);
        let prompt = format!("{}\n{}", system_prompt, message);
        let mut stream = self.provider.chat(prompt);
        let mut stdout = io::stdout();
        let ctrl_c = signal::ctrl_c();
        tokio::pin!(ctrl_c);

        loop {
            tokio::select! {
                chunk = stream.next() => {
                    match chunk {
                        Some(ChatChunk::Token(t)) => {
                            stdout.write_all(t.as_bytes()).await?;
                            stdout.flush().await?;
                        }
                        Some(ChatChunk::ToolCall(_)) => {
                            // placeholder for tool call handling
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

#[cfg(test)]
mod tests {
    use super::*;
    use aider_llm::ChatChunk;
    use aider_llm::Usage;
    use std::sync::{Arc, Mutex};
    use tokio::sync::mpsc;
    use tokio_stream::wrappers::ReceiverStream;
    use tokio_stream::StreamExt;

    #[tokio::test]
    async fn session_uses_mock_provider() {
        let provider = MockProvider::new_with_tokens(vec!["hi".into()]);
        let session = Session::with_provider(
            "mock".into(),
            None,
            None,
            false,
            Mode::Code,
            Box::new(provider.clone()),
        );
        let stream = provider.chat("ignored".into());
        let chunks: Vec<ChatChunk> = stream.collect().await;
        let tokens: Vec<String> = chunks
            .into_iter()
            .filter_map(|c| match c {
                ChatChunk::Token(t) => Some(t),
                _ => None,
            })
            .collect();
        assert_eq!(tokens, vec!["hi"]);
        // ensure session can call stream_reply without error
        session
            .stream_reply_with_mode(Mode::Code, "test".into())
            .await
            .unwrap();
    }

    #[derive(Clone)]
    struct CaptureProvider {
        last: Arc<Mutex<Option<String>>>,
    }

    impl ModelProvider for CaptureProvider {
        fn chat(&self, prompt: String) -> ReceiverStream<ChatChunk> {
            *self.last.lock().unwrap() = Some(prompt);
            let (tx, rx) = mpsc::channel(1);
            drop(tx);
            ReceiverStream::new(rx)
        }

        fn usage(&self) -> Usage {
            Usage::default()
        }
    }

    #[tokio::test]
    async fn mode_affects_prompt() {
        let last = Arc::new(Mutex::new(None));
        let provider = CaptureProvider { last: last.clone() };
        let mut session = Session::with_provider(
            "mock".into(),
            None,
            None,
            false,
            Mode::Code,
            Box::new(provider),
        );
        session.handle_message("hi".into()).await.unwrap();
        let prompt1 = last.lock().unwrap().clone().unwrap();
        assert!(prompt1.starts_with(Mode::Code.system_prompt()));

        session.handle_message("/mode help".into()).await.unwrap();
        session.handle_message("question".into()).await.unwrap();
        let prompt2 = last.lock().unwrap().clone().unwrap();
        assert!(prompt2.starts_with(Mode::Help.system_prompt()));
    }
}
