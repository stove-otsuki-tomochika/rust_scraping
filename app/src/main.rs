fn main() {
    let mut input = String::new();

    println!("何か入力してね:");

    std::io::stdin()
        .read_line(&mut input)
        .expect("入力値が読み取れませんでした。");

    println!("あなたが入力したのはこれだ！-> {}", input.trim());
}
