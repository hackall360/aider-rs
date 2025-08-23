use std::path::Path;

/// Convert a file path within the `website` directory to a public URL.
/// Returns an empty string if the path does not belong to the website docs
/// or points into an excluded `_includes` directory.
pub fn fname_to_url(filepath: &str) -> String {
    if filepath.is_empty() {
        return String::new();
    }

    // Normalise path separators
    let filepath = filepath.replace("\\", "/");
    let parts: Vec<String> = Path::new(&filepath)
        .components()
        .map(|c| c.as_os_str().to_string_lossy().into_owned())
        .collect();

    // Locate the `website` directory in the path
    let website_index = match parts.iter().position(|p| p.eq_ignore_ascii_case("website")) {
        Some(idx) => idx,
        None => return String::new(),
    };

    let relevant = &parts[website_index + 1..];
    if !relevant.is_empty() && relevant[0].eq_ignore_ascii_case("_includes") {
        return String::new();
    }

    let mut url_path = relevant.join("/");
    if url_path.to_lowercase().ends_with("index.md") {
        url_path.truncate(url_path.len() - "index.md".len());
    } else if url_path.to_lowercase().ends_with(".md") {
        url_path.truncate(url_path.len() - ".md".len());
        url_path.push_str(".html");
    }

    let url_path = url_path.trim_matches('/');
    if url_path.is_empty() {
        "https://aider.chat".to_string()
    } else {
        format!("https://aider.chat/{}", url_path)
    }
}

/// A simple document used by the help retriever.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Doc {
    pub text: String,
    pub url: Option<String>,
}

/// Trait for retrieving relevant documentation nodes.
pub trait Retriever: Send + Sync {
    fn retrieve(&self, question: &str) -> Vec<Doc>;
}

/// Helper struct to query documentation and format the response.
pub struct Help {
    pub retriever: Box<dyn Retriever>,
}

impl Help {
    /// Create a new help instance from the provided retriever.
    pub fn new<R: Retriever + 'static>(retriever: R) -> Self {
        Self {
            retriever: Box::new(retriever),
        }
    }

    /// Ask a question and format the retrieved documents.
    pub fn ask(&self, question: &str) -> String {
        let nodes = self.retriever.retrieve(question);
        let mut context = format!("# Question: {}\n\n# Relevant docs:\n\n", question);
        for node in nodes {
            let url_attr = node
                .url
                .as_ref()
                .map(|u| format!(" from_url=\"{}\"", u))
                .unwrap_or_default();
            context.push_str(&format!("<doc{}>\n{}\n</doc>\n\n", url_attr, node.text));
        }
        context
    }
}

struct EmptyRetriever;

impl Retriever for EmptyRetriever {
    fn retrieve(&self, _question: &str) -> Vec<Doc> {
        Vec::new()
    }
}

impl Default for Help {
    fn default() -> Self {
        Self::new(EmptyRetriever)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fname_to_url_basic() {
        assert_eq!(
            fname_to_url("website/docs/index.md"),
            "https://aider.chat/docs"
        );
        assert_eq!(
            fname_to_url("website/docs/usage.md"),
            "https://aider.chat/docs/usage.html"
        );
        assert_eq!(
            fname_to_url("website/_includes/header.md"),
            ""
        );
    }
}
