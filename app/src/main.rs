mod scrape;
mod cli;

use std::io;
use cli::{io::open_stdin, state_machine::{self, CliStateMachine}};
use scrape::scrape::{get_html, generate_selector};

#[tokio::main]
async fn main() {
    let mut state_machine = CliStateMachine::new(Box::new(io::stdin().lock()));
    loop {
        state_machine = state_machine.execute().await;
        if let CliStateMachine::Exit(_)  = state_machine {
            break;
        }
    }
}
