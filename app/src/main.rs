mod scrape;
mod cli;

use std::io;
use cli::open_stdin;
use scrape::scrape::{get_html, generate_selector};

#[tokio::main]
async fn main() {
    loop {
        let mut input = String::new();

        println!("何か入力して。終了するときは「exit」って入れてね:");

        open_stdin(io::stdin().lock(),&mut input);

        if input.trim() == "exit" {
            println!("終了するよ！バイバイ！");
            break;
        }
        println!("あなたが入力したのはこれだ！-> {}", input.trim());
        println!("ついでにスクレイピングしてみるね！");
        let fragment = get_html("https://example.com/").await.expect("HTML のデータ取得に失敗しました。");
        let selector = generate_selector("h1").expect("h1 タグのセレクタの生成に失敗しました。");

        for element in fragment.select(&selector) {
            // example.com の h1 タグの中身は "Example Domain" なので、これを検証する
            println!("{}", element.inner_html());
        }
    }
}
