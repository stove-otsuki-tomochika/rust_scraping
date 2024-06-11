#[cfg(test)]
mod tests {
    use reqwest;
    use tokio;

    #[tokio::test]
    async fn test_fetch_entire_html_tag() {
        let url = "https://example.com/";
        let rc = reqwest::get(url).await.unwrap();
        let contents = rc.text().await.unwrap();
        println!("{:#?}", contents);
    }
}