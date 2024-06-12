enum CliState {
    Start,
    Running,
    Exit,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_cli_state() {
        use super::*;
        let state = CliState::Start;
        match state {
            CliState::Start => {
                println!("Start");
            }
            CliState::Running => {
                println!("Running");
            }
            CliState::Exit => {
                println!("Exit");
            }
        }
    }
}