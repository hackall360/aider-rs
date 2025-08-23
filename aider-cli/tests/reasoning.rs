use aider_cli::reasoning::{format_reasoning_content, remove_reasoning_content, replace_reasoning_tags};

#[test]
fn reasoning_roundtrip() {
    let tagged = format_reasoning_content("secret", "tag");
    let replaced = replace_reasoning_tags(&tagged, "tag");
    assert!(replaced.contains("THINKING"));
    let cleaned = remove_reasoning_content(&tagged, Some("tag"));
    assert_eq!(cleaned, "");
}
