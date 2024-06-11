use scraper::{Html, Selector};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::get_entire_html_tag_text;

    #[tokio::test]
    async fn sample_test() {
        let html = get_entire_html_tag_text("https://example.com/".to_string()).await.expect("スクレイピング対象サイトのアクセスに失敗しました。");
        
        let fragment = Html::parse_fragment(&html);
        let selector = Selector::parse("h1").unwrap();
        
        for element in fragment.select(&selector) {
            // example.com の h1 タグの中身は "Example Domain" なので、これを検証する
            assert_eq!(element.inner_html(), "Example Domain");
        }
    }
}