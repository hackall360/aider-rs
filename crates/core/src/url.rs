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

fn extract_text(html: &str) -> String {
    let document = Html::parse_document(html);
    // Grab the body text as a rough readability extraction
    let selector = Selector::parse("body").unwrap();
    document
        .select(&selector)
        .next()
        .map(|el| el.text().collect::<Vec<_>>().join(" "))
        .unwrap_or_default()
}
