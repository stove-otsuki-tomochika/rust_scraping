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

    // open_stdin を使うと、標準入力が開ける
    #[test]
    fn test_open_stdin() {
        let cursor = Cursor::new("テスト入力");
        let mut input = String::new();
        open_stdin(cursor, &mut input);
        assert_eq!(input, "テスト入力");
    }
}