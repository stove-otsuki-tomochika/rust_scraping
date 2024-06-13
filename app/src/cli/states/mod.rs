use std::{io::BufRead, vec};

use exit::Exit;
use waiting::Waiting;
pub mod waiting;
pub mod running;
pub mod exit;

pub struct CliState<T> {
    _state:T,
    pub input: String,
    pub html: vec::Vec<String>,
    stdin: Box<dyn BufRead>,
}
pub enum Transitioning {
    Exit(CliState<Exit>),
    Waiting(CliState<Waiting>),
}
