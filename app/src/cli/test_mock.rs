use std::io::Cursor;

// 標準入力する値を受け取り、
// その入力値が入力された状態の mock を生成する
pub fn stdin_mock_with_inputted_text(input: &str) -> Cursor<&str> {
    Cursor::new(input)
}