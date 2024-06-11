use scraper::{Html, Selector};
use anyhow::{anyhow, Context,Result};
use super::request::get_entire_html_tag_text;

async fn get_html_fragment_from_url(url: &str) -> Result<Html> {
    let html = get_entire_html_tag_text(url)
        .await
        .context("スクレイピング対象サイトのアクセスに失敗しました。")?;
    Ok(Html::parse_fragment(&html))
}

fn generate_selector(css_selector: &str) -> Result<Selector> {
    Selector::parse(css_selector)
        .map_err(|e| anyhow!("CSS セレクタのパースに失敗しました: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn sample_test() -> Result<()> {
        let fragment = get_html_fragment_from_url("https://example.com/").await?;
        let selector = generate_selector("h1")?;

        for element in fragment.select(&selector) {
            // example.com の h1 タグの中身は "Example Domain" なので、これを検証する
            assert_eq!(element.inner_html(), "Example Domain");
        }
        Ok(())
    }
}