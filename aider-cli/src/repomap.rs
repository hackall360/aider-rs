use std::fs;
use std::path::Path;

use anyhow::Result;
use tree_sitter::{Parser, Tree};
use tree_sitter_highlight::{HighlightConfiguration, Highlighter};

/// Port of the Python `repomap.py` using `tree-sitter`.
pub struct RepoMap {
    parser: Parser,
    #[allow(dead_code)]
    highlight: HighlightConfiguration,
}

impl RepoMap {
    /// Create a new `RepoMap` using the Python grammar as a default example.
    pub fn new() -> Result<Self> {
        let language = tree_sitter_python::language();
        let mut parser = Parser::new();
        parser.set_language(language)?;

        let highlight =
            HighlightConfiguration::new(language, tree_sitter_python::HIGHLIGHT_QUERY, "", "")?;

        Ok(Self { parser, highlight })
    }

    /// Parse a source file into a syntax tree.
    pub fn parse_file(&mut self, path: &Path) -> Result<Tree> {
        let src = fs::read_to_string(path)?;
        self.parser
            .parse(&src, None)
            .ok_or_else(|| anyhow::anyhow!("failed to parse"))
    }

    /// Highlight a string using the configured grammar.
    #[allow(dead_code)]
    pub fn highlight(&self, source: &str) -> Result<Vec<String>> {
        let mut highlighter = Highlighter::new();
        let mut ranges = Vec::new();
        let bytes = source.as_bytes();
        for event in highlighter.highlight(&self.highlight, bytes, None, |_| None)? {
            if let tree_sitter_highlight::HighlightEvent::Source { start, end } = event? {
                ranges.push(String::from_utf8_lossy(&bytes[start..end]).into_owned());
            }
        }
        Ok(ranges)
    }
}
