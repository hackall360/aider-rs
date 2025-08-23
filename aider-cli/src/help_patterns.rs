/// Patterns of files to exclude when bundling website help content.
/// Ported from the Python `help_pats.py` module.
pub const EXCLUDE_WEBSITE_PATS: &[&str] = &[
    "**/.DS_Store",
    "examples/**",
    "_posts/**",
    "HISTORY.md",
    "docs/benchmarks*md",
    "docs/ctags.md",
    "docs/unified-diffs.md",
    "docs/leaderboards/index.md",
    "assets/**",
    ".jekyll-metadata",
    "Gemfile.lock",
    "Gemfile",
    "_config.yml",
    "**/OLD/**",
    "OLD/**",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn patterns_include_known_entry() {
        assert!(EXCLUDE_WEBSITE_PATS.contains(&"HISTORY.md"));
    }
}
