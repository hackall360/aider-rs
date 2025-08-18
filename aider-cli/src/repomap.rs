use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tree_sitter::{Language, Node, Parser};

/// A single symbol extracted from the repository.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symbol {
    pub file: String,
    pub kind: String,
    pub name: String,
    pub signature: String,
    pub line: usize,
}

/// Lightweight repository map built using tree-sitter.
struct FileInfo {
    path: PathBuf,
    symbols: Vec<Symbol>,
    tokens: usize,
    deps: Vec<String>,
}

/// Lightweight repository map built using tree-sitter.
pub struct RepoMap {
    parser: Parser,
    index: Vec<Symbol>,
    max_tokens: usize,
    tokens: usize,
    damping: f64,
    iterations: usize,
}

impl RepoMap {
    /// Create an empty repository map with a token budget.
    pub fn new(max_tokens: usize) -> Self {
        Self::new_with(max_tokens, 0.85, 20)
    }

    /// Create a repository map with explicit ranking parameters.
    pub fn new_with(max_tokens: usize, damping: f64, iterations: usize) -> Self {
        Self {
            parser: Parser::new(),
            index: Vec::new(),
            max_tokens,
            tokens: 0,
            damping,
            iterations,
        }
    }

    /// Build the map from a list of files.
    pub fn build(&mut self, files: &[PathBuf]) -> Result<()> {
        let mut infos = Vec::new();
        for path in files {
            infos.push(self.parse_file(path)?);
        }

        let mut stem_to_idx = HashMap::new();
        for (i, info) in infos.iter().enumerate() {
            if let Some(stem) = info.path.file_stem().and_then(|s| s.to_str()) {
                stem_to_idx.insert(stem.to_string(), i);
            }
        }

        let mut edges: Vec<Vec<usize>> = vec![Vec::new(); infos.len()];
        for (i, info) in infos.iter().enumerate() {
            for dep in &info.deps {
                if let Some(&j) = stem_to_idx.get(dep) {
                    edges[i].push(j);
                }
            }
        }

        let n = infos.len();
        if n == 0 {
            return Ok(());
        }
        let mut scores = vec![1.0 / n as f64; n];
        for _ in 0..self.iterations {
            let mut new_scores = vec![(1.0 - self.damping) / n as f64; n];
            for (i, outs) in edges.iter().enumerate() {
                if outs.is_empty() {
                    let share = self.damping * scores[i] / n as f64;
                    for ns in &mut new_scores {
                        *ns += share;
                    }
                } else {
                    let share = self.damping * scores[i] / outs.len() as f64;
                    for &j in outs {
                        new_scores[j] += share;
                    }
                }
            }
            scores = new_scores;
        }

        let mut order: Vec<usize> = (0..n).collect();
        order.sort_by(|&a, &b| {
            scores[b]
                .partial_cmp(&scores[a])
                .unwrap()
                .then_with(|| infos[a].path.cmp(&infos[b].path))
        });

        for idx in order {
            let info = &infos[idx];
            if self.tokens + info.tokens > self.max_tokens {
                break;
            }
            self.tokens += info.tokens;
            self.index.extend(info.symbols.clone());
        }
        Ok(())
    }

    fn parse_file(&mut self, path: &Path) -> Result<FileInfo> {
        let src = fs::read_to_string(path)?;
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        let deps = extract_deps(&src);
        let mut symbols = Vec::new();
        let mut tokens = 0;
        if let Some(lang) = language_for_extension(ext) {
            self.parser.set_language(lang)?;
            if let Some(tree) = self.parser.parse(&src, None) {
                let root = tree.root_node();
                extract_symbols(path, &src, root, &mut symbols, &mut tokens);
            }
        }
        Ok(FileInfo {
            path: path.to_path_buf(),
            symbols,
            tokens,
            deps,
        })
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

fn extract_symbols(
    path: &Path,
    src: &str,
    node: Node,
    symbols: &mut Vec<Symbol>,
    tokens: &mut usize,
) {
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
        *tokens += cost;
        symbols.push(symbol);
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        extract_symbols(path, src, child, symbols, tokens);
    }
}

fn extract_deps(src: &str) -> Vec<String> {
    let mut deps = Vec::new();
    for line in src.lines() {
        let line = line.trim_start();
        if line.starts_with("use ") || line.starts_with("mod ") || line.starts_with("import ") {
            if let Some(rest) = line.split_whitespace().nth(1) {
                let base = rest
                    .trim_start_matches("crate::")
                    .trim_start_matches("super::");
                let part = base.split("::").next().unwrap_or("");
                let name = part
                    .split(|c: char| c == ';' || c == '{' || c == '(' || c == '"' || c == '\'' )
                    .next()
                    .unwrap_or("");
                if !name.is_empty() {
                    deps.push(name.to_string());
                }
            }
        }
    }
    deps
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
