use std::io::BufRead;

pub fn open_stdin<T: BufRead>(mut reader: T, input: &mut String) {
    reader
    .read_line( input)
    .expect("入力値が読み取れませんでした。");
}

#[cfg(test)]
mod tests {
    use crate::cli::test_mock::stdin_mock_with_inputted_text;
    use super::*;

    // open_stdin を使うと、標準入力が開ける
    #[test]
    fn test_open_stdin() {
        let stdin_mock = stdin_mock_with_inputted_text("テスト入力");
        let mut input_from_user = String::new();

        open_stdin(stdin_mock, &mut input_from_user);
        assert_eq!(input_from_user, "テスト入力");
    }
}