mod scrape;
mod cli;

use std::io;
use cli::state_machine::CliStateMachine;

#[tokio::main]
async fn main() {
    let mut state_machine = CliStateMachine::new(Box::new(io::stdin().lock()));
    loop {
        state_machine = state_machine.execute().await;
        if let CliStateMachine::Exit(_)  = state_machine {
            println!("終了します。");
            break;
        }
    }
}
