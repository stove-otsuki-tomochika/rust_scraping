use scraper::{Html, Selector};
use anyhow::{anyhow, Context,Result};
use super::request::get_entire_html_tag_text;

async fn get_html(url: &str) -> Result<Html> {
    let html = get_entire_html_tag_text(url)
        .await
        .context("スクレイピング対象サイトのアクセスに失敗しました。")?;
    Ok(Html::parse_fragment(&html))
}

fn generate_selector(css_selector: &str) -> Result<Selector> {
    Selector::parse(css_selector)
        .map_err(|e| anyhow!("CSS セレクタのパースに失敗しました: {}", e))
}

pub async fn fetch_inner_html_into_vec(url: &str, css_selector: &str) -> Result<Vec<String>> {
    let fragment = get_html(url).await?;
    let selector = generate_selector(css_selector)?;

    let mut inner_html = vec![];
    for element in fragment.select(&selector) {
        inner_html.push(element.inner_html());
    }
    Ok(inner_html)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_h1_extraction_with_get_html_and_generate_selector() -> Result<()> {
        let fragment = get_html("https://example.com/").await?;
        let selector = generate_selector("h1")?;

        for element in fragment.select(&selector) {
            // example.com の h1 タグの中身は "Example Domain" なので、これを検証する
            assert_eq!(element.inner_html(), "Example Domain");
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_fetch_inner_html_into_vec() -> Result<()> {
        let inner_html = fetch_inner_html_into_vec("https://example.com/", "h1").await?;
        assert_eq!(inner_html, vec!["Example Domain"]);

        Ok(())
    }
}