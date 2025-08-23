use regex::Regex;

/// Standard reasoning tag identifier.
pub const REASONING_TAG: &str = "thinking-content-7bbeb8e1441453ad999a0bbba8a46d4b";

/// Markers used when rendering reasoning content for display.
pub const REASONING_START: &str = "--------------\n► **THINKING**";
pub const REASONING_END: &str = "------------\n► **ANSWER**";

/// Remove reasoning content from text based on tags.
pub fn remove_reasoning_content(res: &str, reasoning_tag: Option<&str>) -> String {
    if reasoning_tag.is_none() {
        return res.to_string();
    }
    let tag = reasoning_tag.unwrap();
    let pattern = Regex::new(&format!("<{}>.*?</{}>", tag, tag)).unwrap();
    let mut res = pattern.replace_all(res, "").to_string();
    let closing = format!("</{}>", tag);
    if let Some(idx) = res.find(&closing) {
        res = res[idx + closing.len()..].to_string();
    }
    res.trim().to_string()
}

/// Replace opening and closing reasoning tags with standard formatting.
pub fn replace_reasoning_tags(text: &str, tag_name: &str) -> String {
    if text.is_empty() {
        return String::new();
    }
    let open = Regex::new(&format!("\\s*<{}>\\s*", tag_name)).unwrap();
    let close = Regex::new(&format!("\\s*</{}>\\s*", tag_name)).unwrap();
    let text = open.replace_all(text, &format!("\n{}\n\n", REASONING_START));
    let text = close.replace_all(&text, &format!("\n\n{}\n\n", REASONING_END));
    text.into_owned()
}

/// Format reasoning content with tags.
pub fn format_reasoning_content(reasoning_content: &str, tag_name: &str) -> String {
    if reasoning_content.is_empty() {
        return String::new();
    }
    format!("<{}>\n\n{}\n\n</{}>", tag_name, reasoning_content, tag_name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_reasoning_content() {
        let text = "<tag>secret</tag> public";
        assert_eq!(remove_reasoning_content(text, Some("tag")), "public");
    }

    #[test]
    fn test_replace_reasoning_tags() {
        let text = "<tag>reasoning</tag>";
        let replaced = replace_reasoning_tags(text, "tag");
        assert!(replaced.contains(REASONING_START));
        assert!(replaced.contains(REASONING_END));
    }

    #[test]
    fn test_format_reasoning_content() {
        let formatted = format_reasoning_content("content", "tag");
        assert_eq!(formatted, "<tag>\n\ncontent\n\n</tag>");
    }
}

