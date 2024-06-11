#[cfg(test)]
mod tests {
    use reqwest;
    use tokio;
    use anyhow::{Context, Result};

    #[tokio::test]
    async fn test_fetch_entire_html_tag() -> Result<()> {
        let url = "https://example.com/";
        let rc = reqwest::get(url).await.context("スクレイピング対象サイトの html タグ(全体)を取得できませんでした。")?;
        let contents = rc.text().await.context("html タグ -> String の変換に失敗しました")?;
        println!("{:#?}", contents);
        Ok(())
    }
}