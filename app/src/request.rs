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

        // 開始タグと終了タグが取得できているか確認することで、
        // 疑似的に全体が取得出来たことを確認する
        assert!(contents.contains("<html>"),"html 開始タグが取得できませんでした。");
        assert!(contents.contains("</html>"),"html 閉じタグが取得できませんでした。");
        Ok(())
    }
}