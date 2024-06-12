pub mod state;
use std::io::BufRead;

pub fn open_stdin<T: BufRead>(mut reader: T, input: &mut String) {
    reader
    .read_line( input)
    .expect("入力値が読み取れませんでした。");
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    // 標準入力する値を受け取り、
    // その入力値が入力された状態の mock を生成する
    fn stdin_mock_with_inputted_text(input: &str) -> Cursor<&str> {
        Cursor::new(input)
    }

    // open_stdin を使うと、標準入力が開ける
    #[test]
    fn test_open_stdin() {
        let stdin_mock = stdin_mock_with_inputted_text("テスト入力");
        let mut input_from_user = String::new();

        open_stdin(stdin_mock, &mut input_from_user);
        assert_eq!(input_from_user, "テスト入力");
    }
}