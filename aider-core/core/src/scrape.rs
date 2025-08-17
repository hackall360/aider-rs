use anyhow::Result;
use reqwest::Client;
use scraper::{Html, Selector};

#[cfg(feature = "pandoc")]
use pandoc::{self, InputFormat, InputKind, OutputFormat, OutputKind, PandocOutput};

#[cfg(feature = "playwright")]
use anyhow::anyhow;
#[cfg(feature = "playwright")]
use tokio::process::Command;

/// Fetches the given URL and returns Markdown content.
///
/// If the `playwright` feature is enabled and Playwright is installed,
/// it will be used to render the page before conversion. Otherwise a
/// simple HTTP fetch is performed.
pub async fn scrape_url(url: &str) -> Result<String> {
    let html = {
        #[cfg(feature = "playwright")]
        {
            if let Ok(h) = fetch_with_playwright(url).await {
                h
            } else {
                fetch_with_reqwest(url).await?
            }
        }
        #[cfg(not(feature = "playwright"))]
        {
            fetch_with_reqwest(url).await?
        }
    };
    html_to_markdown(&html)
}

async fn fetch_with_reqwest(url: &str) -> Result<String> {
    let resp = Client::new().get(url).send().await?;
    Ok(resp.text().await?)
}

#[cfg(feature = "playwright")]
async fn fetch_with_playwright(url: &str) -> Result<String> {
    let script = format!(
        "const {{chromium}} = require('playwright');(async()=>{{const b=await chromium.launch();const p=await b.newPage();await p.goto('{}');console.log(await p.content());await b.close();}})();",
        url
    );
    let out = Command::new("node").arg("-e").arg(script).output().await?;
    if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).to_string())
    } else {
        Err(anyhow!(String::from_utf8_lossy(&out.stderr)))
    }
}

fn html_to_markdown(html: &str) -> Result<String> {
    #[cfg(feature = "pandoc")]
    if let Ok(md) = pandoc_html_to_md(html) {
        return Ok(md);
    }
    Ok(basic_html_to_md(html))
}

#[cfg(feature = "pandoc")]
fn pandoc_html_to_md(html: &str) -> Result<String> {
    let mut p = pandoc::new();
    p.set_input(InputKind::Str(html.into()));
    p.set_input_format(InputFormat::Html, vec![]);
    p.set_output_format(OutputFormat::Markdown, vec![]);
    p.set_output(OutputKind::Pipe);
    let PandocOutput { stdout, .. } = p.execute()?;
    Ok(stdout)
}

fn basic_html_to_md(html: &str) -> String {
    let document = Html::parse_document(html);
    let selector = Selector::parse("h1,h2,h3,p,li").unwrap();
    let mut out = String::new();
    for el in document.select(&selector) {
        let text = el.text().collect::<Vec<_>>().join(" ").trim().to_string();
        if text.is_empty() { continue; }
        match el.value().name() {
            "h1" => out.push_str(&format!("# {}\n\n", text)),
            "h2" => out.push_str(&format!("## {}\n\n", text)),
            "h3" => out.push_str(&format!("### {}\n\n", text)),
            "li" => out.push_str(&format!("- {}\n", text)),
            _ => out.push_str(&format!("{}\n\n", text)),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::basic_html_to_md;

    #[test]
    fn converts_basic_html() {
        let html = "<h1>Title</h1><p>Hello</p><ul><li>Item</li></ul>";
        let md = basic_html_to_md(html);
        assert!(md.contains("# Title"));
        assert!(md.contains("Hello"));
        assert!(md.contains("- Item"));
    }
}
