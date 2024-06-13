use reqwest;
use anyhow::{Context, Result};

pub async fn get_entire_html_tag_text(url: &str) -> Result<String> {
    let rc = reqwest::get(url).await.context("スクレイピング対象サイトのアクセスに失敗しました。")?;
    let contents = rc.text().await.context("html タグ -> String の変換に失敗しました。")?;
    Ok(contents)
}

#[cfg(test)]
mod tests {
    use tokio;
    use super::*;

    #[tokio::test]
    async fn test_fetch_entire_html_tag() -> Result<()> {
        let entire_html_tag = get_entire_html_tag_text("https://example.com/").await.
            context("スクレイピング対象サイトの html タグが取得できませんでした。")?;

        // 開始タグと終了タグが取得できているか確認することで、
        // 疑似的に全体が取得出来たことを確認する
        assert!(entire_html_tag.contains("<html>"),"html 開始タグが存在していません。");
        assert!(entire_html_tag.contains("</html>"),"html 閉じタグが存在していません。");
        Ok(())
    }
}