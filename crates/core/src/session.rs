use anyhow::{anyhow, Result};
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt};
use tokio::process::Command as TokioCommand;
use tokio::signal;
use tokio_stream::StreamExt;
use tracing::{debug, debug_span};

use crate::command::{self, Command};
use crate::voice::VoiceTranscriber;
use aider_llm::{get_provider, mock::MockProvider, ChatChunk, ModelProvider};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

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
    root: PathBuf,
    files: HashSet<PathBuf>,
    urls: HashMap<String, String>,
    images: HashMap<PathBuf, String>,
    state_path: PathBuf,
    no_lint: bool,
    no_test: bool,
    max_fix_attempts: u32,
    voice: Option<VoiceTranscriber>,
}

#[derive(Serialize, Deserialize, Default)]
struct SessionState {
    model: String,
    files: Vec<PathBuf>,
    urls: Vec<String>,
}

impl Session {
    /// Create a new session with the chosen options.
    pub fn new(
        model_name: String,
        prompt: Option<String>,
        api_key: Option<String>,
        dry_run: bool,
        mode: Mode,
        no_lint: bool,
        no_test: bool,
        max_fix_attempts: u32,
        voice: Option<VoiceTranscriber>,
    ) -> Self {
        let root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        Self::with_provider(
            model_name,
            prompt,
            api_key,
            dry_run,
            mode,
            no_lint,
            no_test,
            max_fix_attempts,
            voice,
            Box::new(MockProvider::default()),
            root,
        )
    }

    pub fn with_provider(
        model_name: String,
        prompt: Option<String>,
        api_key: Option<String>,
        dry_run: bool,
        mode: Mode,
        no_lint: bool,
        no_test: bool,
        max_fix_attempts: u32,
        voice: Option<VoiceTranscriber>,
        provider: Box<dyn ModelProvider>,
        root: PathBuf,
    ) -> Self {
        let state_path = root.join(".aider.session");
        let mut session = Self {
            model_name,
            prompt,
            api_key,
            dry_run,
            history: Vec::new(),
            provider,
            mode,
            root,
            files: HashSet::new(),
            urls: HashMap::new(),
            images: HashMap::new(),
            state_path,
            no_lint,
            no_test,
            max_fix_attempts,
            voice,
        };
        session.load_state();
        session
    }

    fn load_state(&mut self) {
        if let Ok(data) = std::fs::read_to_string(&self.state_path) {
            if let Ok(state) = serde_yaml::from_str::<SessionState>(&data) {
                self.model_name = state.model;
                self.files = state.files.into_iter().collect();
                self.urls = state.urls.into_iter().map(|u| (u, String::new())).collect();
            }
        }
    }

    fn save_state(&self) {
        let state = SessionState {
            model: self.model_name.clone(),
            files: self.files.iter().cloned().collect(),
            urls: self.urls.keys().cloned().collect(),
        };
        if let Ok(text) = serde_yaml::to_string(&state) {
            let _ = std::fs::write(&self.state_path, text);
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
                if let Some(v) = &self.voice {
                    match v.record_and_transcribe().await {
                        Ok(text) if !text.is_empty() => {
                            println!("{text}");
                            self.handle_message(text).await?;
                        }
                        Ok(_) => println!("[voice: no speech detected]"),
                        Err(err) => println!("[voice error: {err}]"),
                    }
                }
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
        if let Some(cmd) = command::parse(&message) {
            let out = match self.handle_command(cmd).await {
                Ok(s) => s,
                Err(err) => err.to_string(),
            };
            if !out.is_empty() {
                println!("{out}");
            }
            return Ok(());
        }
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
        if message.starts_with('/') {
            let name = message[1..].split_whitespace().next().unwrap_or("");
            println!("unknown command: {name}");
            return Ok(());
        }
        self.history.push(message.clone());
        self.stream_reply_with_mode(self.mode.clone(), message)
            .await?;
        Ok(())
    }

    async fn handle_command(&mut self, cmd: Command) -> Result<String> {
        let span = debug_span!("command", ?cmd);
        let _enter = span.enter();
        match cmd {
            Command::Add(paths) => {
                if paths.is_empty() {
                    return Err(anyhow!("no paths provided"));
                }
                let vision = self.model_name.to_lowercase().contains("vision");
                for p in paths {
                    let abs = self.root.join(&p);
                    let canon = abs
                        .canonicalize()
                        .map_err(|_| anyhow!("invalid path: {}", p.display()))?;
                    if !canon.starts_with(&self.root) {
                        return Err(anyhow!("path outside repository: {}", p.display()));
                    }
                    let rel = canon.strip_prefix(&self.root).unwrap().to_path_buf();
                    if is_image(&canon) {
                        if vision {
                            self.images.insert(rel.clone(), String::new());
                        } else {
                            let text = ocr_image(&canon).await;
                            self.images.insert(rel.clone(), text);
                        }
                    }
                    self.files.insert(rel);
                }
                self.save_state();
                Ok("Files added".into())
            }
            Command::AddUrl(urls) => {
                if urls.is_empty() {
                    return Err(anyhow!("no urls provided"));
                }
                for url in urls {
                    let (text, tokens) = crate::url::fetch(&url).await?;
                    self.urls.insert(url.clone(), text);
                    println!("Fetched {url} (~{tokens} tokens)");
                }
                self.save_state();
                Ok("URLs added".into())
            }
            Command::Drop(paths) => {
                if paths.is_empty() {
                    return Err(anyhow!("no paths provided"));
                }
                for p in paths {
                    let abs = self.root.join(&p);
                    let canon = abs
                        .canonicalize()
                        .map_err(|_| anyhow!("invalid path: {}", p.display()))?;
                    let rel = canon.strip_prefix(&self.root).unwrap().to_path_buf();
                    if !self.files.remove(&rel) {
                        return Err(anyhow!("file not in session: {}", rel.display()));
                    }
                }
                self.save_state();
                Ok("Files dropped".into())
            }
            Command::Model(name) => {
                if name.is_empty() {
                    return Err(anyhow!("no model specified"));
                }
                match get_provider(&name) {
                    Some(p) => {
                        self.model_name = name;
                        self.provider = p;
                        self.save_state();
                        Ok("Model switched".into())
                    }
                    None => Err(anyhow!("unknown model: {}", name)),
                }
            }
            Command::Help => Ok(format!(
                "Commands: /add, /add-url, /drop, /model, /help\nActive model: {}\nFiles tracked: {}\nURLs tracked: {}",
                self.model_name,
                self.files.len(),
                self.urls.len()
            )),
        }
    }

    async fn stream_reply_with_mode(&self, mode: Mode, message: String) -> Result<()> {
        let system_prompt = mode.system_prompt();
        debug!(mode=?mode, prompt=system_prompt);
        let mut context = String::new();
        for file in &self.files {
            let path = self.root.join(file);
            if let Ok(txt) = std::fs::read_to_string(&path) {
                context.push_str(&format!("File: {}\n{}\n", file.display(), txt));
            }
        }
        for (url, text) in &self.urls {
            context.push_str(&format!("URL: {}\n{}\n", url, text));
        }
        for (img, text) in &self.images {
            if text.is_empty() {
                context.push_str(&format!("Image: {}\n", img.display()));
            } else {
                context.push_str(&format!("Image: {}\n{}\n", img.display(), text));
            }
        }
        let prompt = format!("{}\n{}\n{}", system_prompt, context, message);
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

fn is_image(path: &std::path::Path) -> bool {
    match path.extension().and_then(|s| s.to_str()) {
        Some(ext) => matches!(
            ext.to_lowercase().as_str(),
            "png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp"
        ),
        None => false,
    }
}

async fn ocr_image(path: &std::path::Path) -> String {
    if let Ok(out) = TokioCommand::new("tesseract")
        .arg(path)
        .arg("stdout")
        .output()
        .await
    {
        if out.status.success() {
            return String::from_utf8_lossy(&out.stdout).to_string();
        }
    }
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command;
    use aider_llm::ChatChunk;
    use aider_llm::Usage;
    use std::path::PathBuf;
    use std::sync::{Arc, Mutex};
    use tokio::sync::mpsc;
    use tokio_stream::wrappers::ReceiverStream;
    use tokio_stream::StreamExt;

    #[tokio::test]
    async fn session_uses_mock_provider() {
        let dir = tempfile::tempdir().unwrap();
        let provider = MockProvider::new_with_tokens(vec!["hi".into()]);
        let session = Session::with_provider(
            "mock".into(),
            None,
            None,
            false,
            Mode::Code,
            false,
            false,
            1,
            None,
            Box::new(provider.clone()),
            dir.path().to_path_buf(),
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
        let dir = tempfile::tempdir().unwrap();
        let last = Arc::new(Mutex::new(None));
        let provider = CaptureProvider { last: last.clone() };
        let mut session = Session::with_provider(
            "mock".into(),
            None,
            None,
            false,
            Mode::Code,
            false,
            false,
            1,
            None,
            Box::new(provider),
            dir.path().to_path_buf(),
        );
        session.handle_message("hi".into()).await.unwrap();
        let prompt1 = last.lock().unwrap().clone().unwrap();
        assert!(prompt1.starts_with(Mode::Code.system_prompt()));

        session.handle_message("/mode help".into()).await.unwrap();
        session.handle_message("question".into()).await.unwrap();
        let prompt2 = last.lock().unwrap().clone().unwrap();
        assert!(prompt2.starts_with(Mode::Help.system_prompt()));
    }

    #[tokio::test]
    async fn commands_update_state() {
        use std::fs;
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("file.txt"), "hi").unwrap();
        let provider = MockProvider::default();
        let mut session = Session::with_provider(
            "mock".into(),
            None,
            None,
            false,
            Mode::Code,
            false,
            false,
            1,
            None,
            Box::new(provider),
            dir.path().to_path_buf(),
        );

        // add file
        let cmd = command::parse("/add file.txt").unwrap();
        session.handle_command(cmd).await.unwrap();
        assert!(session.files.contains(&PathBuf::from("file.txt")));
        let state = fs::read_to_string(dir.path().join(".aider.session")).unwrap();
        assert!(state.contains("file.txt"));

        // switch model
        let cmd = command::parse("/model mock2").unwrap();
        session.handle_command(cmd).await.unwrap();
        assert_eq!(session.model_name, "mock2");

        // help output
        let cmd = command::parse("/help").unwrap();
        let out = session.handle_command(cmd).await.unwrap();
        assert!(out.contains("mock2"));
        assert!(out.contains("Files tracked: 1"));

        // drop file
        let cmd = command::parse("/drop file.txt").unwrap();
        session.handle_command(cmd).await.unwrap();
        assert!(session.files.is_empty());
        let state = fs::read_to_string(dir.path().join(".aider.session")).unwrap();
        assert!(!state.contains("file.txt"));
    }

    #[tokio::test]
    async fn add_url_fetches_content() {
        use httpmock::prelude::*;
        let dir = tempfile::tempdir().unwrap();
        let server = MockServer::start_async().await;
        let _m = server.mock(|when, then| {
            when.method(GET).path("/page");
            then.status(200)
                .body("<html><body><p>Hello</p></body></html>");
        });
        let provider = MockProvider::default();
        let mut session = Session::with_provider(
            "mock".into(),
            None,
            None,
            false,
            Mode::Code,
            false,
            false,
            1,
            None,
            Box::new(provider),
            dir.path().to_path_buf(),
        );
        let url = format!("{}{}", server.url(""), "/page");
        let cmd = command::parse(&format!("/add-url {}", url)).unwrap();
        session.handle_command(cmd).await.unwrap();
        assert!(session.urls.contains_key(&url));
    }
}
