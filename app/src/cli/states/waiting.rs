use std::io::BufRead;

use super::CliState;

pub struct Waiting {}
impl CliState<Waiting> {
    pub fn new(state:Waiting, stdin:Box<dyn BufRead>) -> Self {
        CliState {
            _state: state,
            input: String::new(),
            stdin: stdin
        }
    }

    // pub fn execute(&mut self) {
    //     self.stdin
    //     .read_line(&mut self.input)
    //     .expect("入力値が読み取れませんでした。");
    // }
}