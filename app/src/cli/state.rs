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
struct Running;
struct Exit;

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
}