use std::{fs, path::Path};

use tree_sitter::{Language, Parser, Query, QueryCursor};

use crate::repo::Repo;

fn language_for(path: &Path) -> Option<Language> {
    match path.extension().and_then(|s| s.to_str()) {
        Some("rs") => Some(tree_sitter_rust::language()),
        Some("go") => Some(tree_sitter_go::language()),
        Some("dart") => Some(tree_sitter_dart::language()),
        _ => None,
    }
}

fn query_for(ext: &str) -> &'static str {
    match ext {
        "rs" => "(function_item name: (identifier) @name)",
        "go" => "(function_declaration name: (identifier) @name)",
        "dart" => "(function_declaration name: (identifier) @name)",
        _ => "",
    }
}

/// Build a simple textual map of functions in the repository using tree-sitter.
pub fn build(repo: &Repo) -> Result<String, anyhow::Error> {
    let mut out = String::new();
    for path in repo.files() {
        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
        let Some(lang) = language_for(&path) else { continue; };
        let mut parser = Parser::new();
        parser.set_language(lang)?;
        let src = fs::read_to_string(&path)?;
        if let Some(tree) = parser.parse(&src, None) {
            let query_src = query_for(ext);
            if query_src.is_empty() {
                continue;
            }
            let query = Query::new(lang, query_src)?;
            let mut cursor = QueryCursor::new();
            for m in cursor.matches(&query, tree.root_node(), src.as_bytes()) {
                for cap in m.captures {
                    let name = cap.node.utf8_text(src.as_bytes()).unwrap_or("<err>");
                    let line = cap.node.start_position().row + 1;
                    let rel = path.strip_prefix(&repo.root).unwrap_or(&path);
                    out.push_str(&format!("{}:{} {}\n", rel.display(), line, name));
                }
            }
        }
    }
    Ok(out)
}

