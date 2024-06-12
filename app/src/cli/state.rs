pub enum CliState {
    Waiting(Waiting),
    Running(Running),
    Exit(Exit),
}

impl CliState {
    pub fn new() -> Self {
        CliState::Waiting(Waiting::new())
    }
}

pub struct Waiting {
    input: String
}
impl Waiting {
    pub fn update(&self) -> CliState {
        if let "exit" = self.input.as_str() {
            return CliState::Exit(Exit {input: self.input.clone()})
        }
        CliState::Running(Running {input: self.input.clone()})
    }

    pub fn input(&self, input: &str) -> Waiting {
        Waiting {input: input.to_string()}
    }

    pub fn new() -> Waiting {
        Waiting {input: String::new()}
    }
}
struct Running {
    input: String
}
struct Exit {
    input: String
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{anyhow, Result};

    #[test]
    fn test_constructor_return_start() -> Result<()> {
        let state = CliState::new();
        match state {
            CliState::Waiting(_) => {
                Ok(())
            }
            CliState::Running(_) => {
                Err(anyhow!("CliState::Running"))
            }
            CliState::Exit(_) => {
                Err(anyhow!("CliState::Exit"))
            }
        }
    }

    #[test]
    fn test_running_state_called_update_from_waiting() -> Result<()> {
        let mut state = CliState::new();
        if let CliState::Waiting(waiting) = &state {
            state = waiting.update();
        }
        match state {
            CliState::Waiting(_) => {
                Err(anyhow!("state「Running」を期待しましたが Waiting でした"))
            }
            CliState::Running(_) => {
                Ok(())
            }
            CliState::Exit(_) => {
                Err(anyhow!("state「Running」を期待しましたが Exit でした"))
            }
        }
    }

    #[test]
    fn test_update_input_field_called_input() {
        let state = Waiting::new();
        let state = state.input("after");
        assert_eq!(state.input, "after");
    }

    #[test]
    fn test_change_exit_state_from_waiting() -> Result<()> {
        let mut state = CliState::new();
        if let CliState::Waiting(waiting) = &state {
            let waiting = waiting.input("exit");
            state = waiting.update();
        }
        match state {
            CliState::Waiting(_) => {
                Err(anyhow!("state「Exit」を期待しましたが Waiting でした"))
            }
            CliState::Running(_) => {
                Err(anyhow!("state「Exit」を期待しましたが Running でした"))
            }
            CliState::Exit(_) => {
                Ok(())
            }
        }
    }
}