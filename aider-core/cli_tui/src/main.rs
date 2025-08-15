use clap::Parser;
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use ratatui::{prelude::*, widgets::{Block, Borders}};
use std::io::{self};

/// Simple TUI that quits on 'q'.
#[derive(Parser)]
struct Args {}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _args = Args::parse();
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default().title("aider").borders(Borders::ALL);
            f.render_widget(block, size);
        })?;
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    disable_raw_mode()?;
    Ok(())
}
