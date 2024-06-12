mod scrape;
mod cli;

use std::io;
use cli::open_stdin;
use scrape::scrape::{get_html, generate_selector};
use cli::state::CliState;

#[tokio::main]
async fn main() {
    loop {
        let mut input_from_user = String::new();
        let cli_state = CliState::new();
        
        match cli_state {
            CliState::Start => {
                println!("何か入力して。終了するときは「exit」って入れてね:");

                open_stdin(io::stdin().lock(),&mut input_from_user);        
            },
            _ => {}
        }
        if input_from_user.trim() == "exit" {
            println!("終了するよ！バイバイ！");
            break;
        }
        println!("あなたが入力したのはこれだ！-> {}", input_from_user.trim());
        println!("ついでにスクレイピングしてみるね！");
        let fragment = get_html("https://example.com/").await.expect("HTML のデータ取得に失敗しました。");
        let selector = generate_selector("h1").expect("h1 タグのセレクタの生成に失敗しました。");

        for element in fragment.select(&selector) {
            // example.com の h1 タグの中身は "Example Domain" なので、これを検証する
            println!("{}", element.inner_html());
        }
    }
}
