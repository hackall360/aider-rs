use aider_core::{fname_to_url, Doc, Help, Retriever};

struct MockRetriever;

impl Retriever for MockRetriever {
    fn retrieve(&self, _question: &str) -> Vec<Doc> {
        let mut docs = Vec::new();
        for i in 0..6 {
            docs.push(Doc {
                text: format!("Doc {} about aider ai chat", i),
                url: Some(format!("https://example.com/doc{}", i)),
            });
        }
        docs
    }
}

#[test]
fn help_init() {
    let help = Help::default();
    // Default retriever should exist and simply return no docs
    assert!(help.retriever.retrieve("test").is_empty());
}

#[test]
fn help_ask_formats_docs() {
    let help = Help::new(MockRetriever);
    let question = "What is aider?";
    let result = help.ask(question);
    assert!(result.contains(&format!("# Question: {}", question)));
    assert!(result.contains("<doc"));
    assert!(result.contains("</doc>"));
    assert!(result.to_lowercase().contains("aider"));
    assert!(result.to_lowercase().contains("ai"));
    assert!(result.to_lowercase().contains("chat"));
    assert!(result.matches("<doc").count() > 5);
    assert!(result.len() > 100);
}

#[test]
fn fname_to_url_unix() {
    assert_eq!(fname_to_url("website/docs/index.md"), "https://aider.chat/docs");
    assert_eq!(fname_to_url("website/docs/usage.md"), "https://aider.chat/docs/usage.html");
    assert_eq!(fname_to_url("website/_includes/header.md"), "");
    assert_eq!(fname_to_url("/home/user/project/website/docs/index.md"), "https://aider.chat/docs");
    assert_eq!(fname_to_url("/home/user/project/website/docs/usage.md"), "https://aider.chat/docs/usage.html");
    assert_eq!(fname_to_url("/home/user/project/website/_includes/header.md"), "");
}

#[test]
fn fname_to_url_windows() {
    assert_eq!(fname_to_url(r"website\docs\index.md"), "https://aider.chat/docs");
    assert_eq!(fname_to_url(r"website\docs\usage.md"), "https://aider.chat/docs/usage.html");
    assert_eq!(fname_to_url(r"website\_includes\header.md"), "");
    assert_eq!(fname_to_url(r"C:\\Users\\user\\project\\website\\docs\\index.md"), "https://aider.chat/docs");
    assert_eq!(fname_to_url(r"C:\\Users\\user\\project\\website\\docs\\usage.md"), "https://aider.chat/docs/usage.html");
    assert_eq!(fname_to_url(r"C:\\Users\\user\\project\\website\\_includes\\header.md"), "");
}

#[test]
fn fname_to_url_edge_cases() {
    assert_eq!(fname_to_url("/home/user/project/docs/index.md"), "");
    assert_eq!(fname_to_url(r"C:\\Users\\user\\project\\docs\\index.md"), "");
    assert_eq!(fname_to_url(""), "");
    assert_eq!(fname_to_url("/home/user/website_project/docs/index.md"), "");
}

