use anyhow::Result;
use reqwest::Url;
use scraper::{Html, Selector};

/// Fetch the contents of a URL and extract readable text.
/// Returns the extracted text and an estimated token count.
pub async fn fetch(url: &str) -> Result<(String, usize)> {
    let url = Url::parse(url)?;
    let resp = reqwest::get(url).await?;
    let body = resp.text().await?;
    let text = extract_text(&body);
    let tokens = text.split_whitespace().count();
    Ok((text, tokens))
}

/// Extract readable text from the provided HTML string.
pub fn extract_text(html: &str) -> String {
    let document = Html::parse_document(html);
    // Grab the body text as a rough readability extraction
    let selector = Selector::parse("body").unwrap();
    document
        .select(&selector)
        .next()
        .map(|el| el.text().collect::<Vec<_>>().join(" "))
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;

    #[test]
    fn extract_simple_text() {
        let html = "<html><body>Hello <b>World</b></body></html>";
        let text = extract_text(html);
        let collapsed = text.split_whitespace().collect::<Vec<_>>().join(" ");
        assert_eq!(collapsed, "Hello World");
    }

    #[tokio::test]
    async fn fetch_returns_text_and_tokens() {
        let server = MockServer::start();
        let body = "<html><body>Test Page</body></html>";
        let mock = server.mock(|when, then| {
            when.method(GET).path("/page");
            then.status(200).body(body);
        });

        let (text, tokens) = fetch(&format!("{}/page", server.url(""))).await.unwrap();
        mock.assert();
        assert_eq!(text.trim(), "Test Page");
        assert_eq!(tokens, 2);
    }
}
