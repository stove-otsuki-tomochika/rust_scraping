use std::io::BufRead;
pub mod waiting;
pub mod running;
pub mod exit;

pub struct CliState<T> {
    _state:T,
    input: String,
    stdin: Box<dyn BufRead>
}
