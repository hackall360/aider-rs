/// Determine whether the given input line is a special slash command.
/// Lines starting with '/' or '!' are considered special commands.
pub fn is_special_command(line: &str) -> bool {
    matches!(line.chars().next(), Some('/') | Some('!'))
}

/// Extract the command name from a special command line, without the prefix.
/// Returns `None` if the line is not a special command.
pub fn extract_command(line: &str) -> Option<&str> {
    if !is_special_command(line) {
        return None;
    }
    line[1..].split_whitespace().next()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_special_command() {
        assert!(is_special_command("/ask"));
        assert!(is_special_command("!help"));
        assert!(!is_special_command("hello"));
    }

    #[test]
    fn test_extract_command() {
        assert_eq!(extract_command("/ask something"), Some("ask"));
        assert_eq!(extract_command("!help"), Some("help"));
        assert_eq!(extract_command("hello"), None);
    }
}
