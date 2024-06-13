use std::{io::BufRead, vec};
pub mod waiting;
pub mod running;
pub mod exit;

pub struct CliState<T> {
    _state:T,
    pub input: String,
    pub html: vec::Vec<String>,
    stdin: Box<dyn BufRead>,
}
