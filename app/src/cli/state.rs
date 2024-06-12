pub enum CliState {
    Waiting(Waiting),
    Running(Running),
    Exit(Exit),
}

impl CliState {
    // コンストラクタ
    pub fn new() -> Self {
        CliState::Waiting(Waiting {input: String::new()})
    }
}

pub struct Waiting {
    input: String
}
impl Waiting {
    pub fn update(&self) -> CliState {
        CliState::Running(Running {input: self.input.clone()})
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
    use anyhow::{anyhow, Result};

    #[test]
    fn test_constructor_return_start() -> Result<()> {
        use super::*;
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
        use super::*;
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
}