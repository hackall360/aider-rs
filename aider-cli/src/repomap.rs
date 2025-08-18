use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tree_sitter::{Language, Node, Parser};

/// A single symbol extracted from the repository.
#[derive(Debug, Serialize, Deserialize)]
pub struct Symbol {
    pub file: String,
    pub kind: String,
    pub name: String,
    pub signature: String,
    pub line: usize,
}

/// Lightweight repository map built using tree-sitter.
pub struct RepoMap {
    parser: Parser,
    index: Vec<Symbol>,
    max_tokens: usize,
    tokens: usize,
}

impl RepoMap {
    /// Create an empty repository map with a token budget.
    pub fn new(max_tokens: usize) -> Self {
        Self {
            parser: Parser::new(),
            index: Vec::new(),
            max_tokens,
            tokens: 0,
        }
    }

    /// Build the map from a list of files.
    pub fn build(&mut self, files: &[PathBuf]) -> Result<()> {
        for path in files {
            if self.tokens >= self.max_tokens {
                break;
            }
            self.index_file(path)?;
        }
        Ok(())
    }

    /// Parse a file and extract symbols.
    pub fn index_file(&mut self, path: &Path) -> Result<()> {
        let src = fs::read_to_string(path)?;
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        if let Some(lang) = language_for_extension(ext) {
            self.parser.set_language(lang)?;
            if let Some(tree) = self.parser.parse(&src, None) {
                let root = tree.root_node();
                self.extract_symbols(path, &src, root);
            }
        }
        Ok(())
    }

    fn extract_symbols(&mut self, path: &Path, src: &str, node: Node) {
        if self.tokens >= self.max_tokens {
            return;
        }

        if let Some(name_node) = node.child_by_field_name("name") {
            let name = name_node
                .utf8_text(src.as_bytes())
                .unwrap_or("")
                .to_string();
            let kind = node.kind().to_string();
            let line = node.start_position().row + 1;
            let line_str = src.lines().nth(line - 1).unwrap_or("").trim().to_string();
            let symbol = Symbol {
                file: path.to_string_lossy().into_owned(),
                kind,
                name,
                signature: line_str.clone(),
                line,
            };
            let cost = line_str.split_whitespace().count();
            if self.tokens + cost <= self.max_tokens {
                self.tokens += cost;
                self.index.push(symbol);
            }
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.extract_symbols(path, src, child);
        }
    }

    /// Print the map to stdout.
    pub fn print(&self) {
        for sym in &self.index {
            println!("{}:{}: {}", sym.file, sym.line, sym.signature);
        }
    }

    /// Persist the map to a JSON file.
    pub fn save(&self, path: &Path) -> Result<()> {
        let data = serde_json::to_string(&self.index)?;
        fs::write(path, data)?;
        Ok(())
    }
}

fn language_for_extension(ext: &str) -> Option<Language> {
    match ext {
        "rs" => Some(tree_sitter_rust::language()),
        "go" => Some(tree_sitter_go::language()),
        "py" => Some(tree_sitter_python::language()),
        "js" | "jsx" => Some(tree_sitter_javascript::language()),
        "ts" => Some(tree_sitter_typescript::language_typescript()),
        _ => None,
    }
}
