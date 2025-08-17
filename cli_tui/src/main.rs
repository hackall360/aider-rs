use std::{io, time::Duration};

use clap::Parser;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{prelude::*, widgets::*};

/// Simple TUI that displays a message and exits when `q` is pressed.
#[derive(Parser, Debug)]
#[command(name = "cli_tui", about = "Ratatui-based CLI")]
struct Cli {
    /// Message to display inside the TUI
    #[arg(short, long, default_value = "Hello from cli_tui! Press q to quit.")]
    message: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| {
            let block = Block::default().title("cli_tui").borders(Borders::ALL);
            let paragraph = Paragraph::new(cli.message.as_str()).block(block);
            f.render_widget(paragraph, f.size());
        })?;
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if matches!(key.code, KeyCode::Char('q') | KeyCode::Esc) {
                    break;
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
