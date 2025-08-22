pub mod search_replace;
pub mod ask;
pub mod help;
pub mod patch;
pub mod udiff;
pub mod wholefile;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoderError {
    #[error("search text not found")]
    NotFound,
}

#[derive(Debug, Clone, Copy)]
pub enum CoderKind {
    Ask,
    Help,
    Patch,
    Udiff,
    WholeFile,
    SearchReplace,
}

pub fn system_prompt(kind: CoderKind) -> &'static str {
    match kind {
        CoderKind::Ask => ask::SYSTEM_PROMPT,
        CoderKind::Help => help::SYSTEM_PROMPT,
        CoderKind::Patch => patch::SYSTEM_PROMPT,
        CoderKind::Udiff => udiff::SYSTEM_PROMPT,
        CoderKind::WholeFile => wholefile::SYSTEM_PROMPT,
        CoderKind::SearchReplace => "",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn system_prompts_load() {
        assert!(system_prompt(CoderKind::Ask).contains("code analyst"));
        assert!(system_prompt(CoderKind::Help).contains("Aider"));
        assert!(system_prompt(CoderKind::Patch).contains("Begin Patch"));
        assert!(system_prompt(CoderKind::Udiff).contains("diff -U0"));
        assert!(system_prompt(CoderKind::WholeFile).contains("Determine if any code changes"));
    }
}
