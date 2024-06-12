pub enum CliState {
    Waiting,
    Running,
    Exit,
}

impl CliState {
    // コンストラクタ
    pub fn new() -> Self {
        CliState::Waiting
    }
}

#[cfg(test)]
mod tests {
    use anyhow::{anyhow, Result};

    #[test]
    fn test_constructor_return_start() -> Result<()> {
        use super::*;
        let state = CliState::new();
        match state {
            CliState::Waiting => {
                Ok(())
            }
            CliState::Running => {
                Err(anyhow!("CliState::Running"))
            }
            CliState::Exit => {
                Err(anyhow!("CliState::Exit"))
            }
        }
    }
}