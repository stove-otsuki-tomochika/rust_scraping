use std::io::BufRead;

use super::{running::Running, CliState};

pub struct Waiting {}
impl CliState<Waiting> {
    pub fn new(state:Waiting, stdin:Box<dyn BufRead>) -> Self {
        CliState {
            _state: state,
            input: String::new(),
            html: vec![],
            stdin: stdin
        }
    }

    pub fn update(self) -> CliState<Running> {
        CliState {_state: Running{}, input:self.input, html: self.html, stdin: self.stdin}
    }

    pub fn execute(&mut self) {
        println!("URLを入力してください。");
        self.stdin
        .read_line(&mut self.input)
        .expect("入力値が読み取れませんでした。");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::test_mock::_stdin_mock_with_inputted_text;
    use anyhow::Result;

    #[tokio::test]
    async fn test_open_stdin_called_execute() -> Result<()> {
        let stdin_mock = _stdin_mock_with_inputted_text("テスト入力");
        let mut waiting = CliState::new(
            Waiting {}, 
            Box::new(stdin_mock)
        );

        waiting.execute();
        assert_eq!(waiting.input, "テスト入力");

        Ok(())
    }
}
