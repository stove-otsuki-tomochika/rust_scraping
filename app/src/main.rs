mod request;

use std::io::{self, BufRead};

fn main() {
    loop {
        let mut input = String::new();

        println!("何か入力してね。終了するときは「exit」って入れてね:");

        io::stdin()
            .lock()
            .read_line(&mut input)
            .expect("入力値が読み取れませんでした。");

        if input.trim() == "exit" {
            println!("終了します。");
            break;
        }
        println!("あなたが入力したのはこれだ！-> {}", input.trim());
    }
}
