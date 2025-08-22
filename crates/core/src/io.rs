use atty::Stream;

/// Ensure a color string has a leading `#` if it looks like a hex value.
pub fn ensure_hash_prefix(color: &str) -> String {
    if color.is_empty() || color.starts_with('#') {
        return color.to_string();
    }
    if color.chars().all(|c| c.is_ascii_hexdigit()) && (color.len() == 3 || color.len() == 6) {
        format!("#{color}")
    } else {
        color.to_string()
    }
}

/// Whether the current stdout is a TTY.
pub fn is_tty() -> bool {
    atty::is(Stream::Stdout)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_prefix() {
        assert_eq!(ensure_hash_prefix("fff"), "#fff");
        assert_eq!(ensure_hash_prefix("#123"), "#123");
        assert_eq!(ensure_hash_prefix(""), "");
    }
}
