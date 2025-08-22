use aider_core::{watch_ask_prompt, watch_code_prompt};

#[test]
fn code_prompt_contains_instructions() {
    let prompt = watch_code_prompt();
    assert!(prompt.contains("I've written your instructions"));
}

#[test]
fn ask_prompt_starts_with_slash() {
    let prompt = watch_ask_prompt();
    assert!(prompt.starts_with("/ask"));
}
