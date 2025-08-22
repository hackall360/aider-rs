/// Prompt displayed when watch mode detects code comments with `AI` markers.
pub const WATCH_CODE_PROMPT: &str = "I've written your instructions in comments in the code and marked them with \"ai\"\nYou can see the \"AI\" comments shown below (marked with █).\nFind them in the code files I've shared with you, and follow their instructions.\n\nAfter completing those instructions, also be sure to remove all the \"AI\" comments from the code too.";

/// Prompt displayed when watch mode detects `/ask` comments.
pub const WATCH_ASK_PROMPT: &str = "/ask\nFind the \"AI\" comments below (marked with █) in the code files I've shared with you.\nThey contain my questions that I need you to answer and other instructions for you.";

/// Return the watch code prompt.
pub fn watch_code_prompt() -> &'static str {
    WATCH_CODE_PROMPT
}

/// Return the watch ask prompt.
pub fn watch_ask_prompt() -> &'static str {
    WATCH_ASK_PROMPT
}
