use std::io::BufRead;

use super::{running::Running, CliState};

pub struct Waiting {}
impl CliState<Waiting> {
    pub fn new(state:Waiting, stdin:Box<dyn BufRead>) -> Self {
        CliState {
            _state: state,
            input: String::new(),
            stdin: stdin
        }
    }

    pub fn update(self) -> CliState<Running> {
        CliState {_state: Running{},input:String::new() , stdin: self.stdin}
    }

    // pub fn execute(&mut self) {
    //     self.stdin
    //     .read_line(&mut self.input)
    //     .expect("入力値が読み取れませんでした。");
    // }
}