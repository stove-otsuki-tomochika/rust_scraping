use scraper::{Html, Selector};

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn sample_test() {
        let html = r#"
            <ul>
                <li>Foo</li>
                <li>Bar</li>
                <li>Baz</li>
            </ul>
        "#;
        
        let fragment = Html::parse_fragment(html);
        let selector = Selector::parse("li").unwrap();
        
        for element in fragment.select(&selector) {
            assert_eq!("li", element.value().name());
        }
    }
}