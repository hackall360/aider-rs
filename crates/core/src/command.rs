use std::path::PathBuf;

/// Parsed representation of a slash command entered by the user.
#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Add(Vec<PathBuf>),
    Drop(Vec<PathBuf>),
    Model(String),
    Help,
}

/// Parse a potential slash command from the given input.
///
/// Returns `None` if the input is not a command.
pub fn parse(input: &str) -> Option<Command> {
    if !input.starts_with('/') {
        return None;
    }
    let mut parts = input[1..].split_whitespace();
    let cmd = parts.next()?.to_lowercase();
    let command = match cmd.as_str() {
        "add" => Command::Add(parts.map(PathBuf::from).collect()),
        "drop" => Command::Drop(parts.map(PathBuf::from).collect()),
        "model" => Command::Model(parts.next().unwrap_or("").to_string()),
        "help" => Command::Help,
        _ => return None,
    };
    Some(command)
}
