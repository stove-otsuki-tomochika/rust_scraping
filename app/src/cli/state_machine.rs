use std::io::BufRead;
use anyhow::{Context,Result};

use crate::scrape::scrape::{generate_selector, get_html};

use super::{io::open_stdin, states::{CliState}};
use super::states::waiting::Waiting;
use super::states::running::Running;
use super::states::exit::Exit;

pub enum CliStateMachine {
    Waiting(CliState<Waiting>),
    Running(CliState<Running>),
    Exit(CliState<Exit>),
}
impl CliStateMachine {
    fn new(stdin: Box<dyn BufRead>) -> Self {
        CliStateMachine::Waiting(CliState::new(Waiting{}, stdin))
    }

    pub fn execute(self) -> Self {
        match self {
            CliStateMachine::Waiting(waiting) => {
                CliStateMachine::Running(waiting.update())
            },
            CliStateMachine::Running(running) => {
                CliStateMachine::Waiting(running.update())
            },
            _ => self
        }
    }

    // pub async fn execute(mut self) -> Self {
    //     self = match self {
    //         CliState::Waiting(mut waiting) => {
    //             waiting.execute();
    //             CliState::Waiting(waiting)
    //         }
    //         CliState::Running(mut running) => {
    //             running.execute().await;
    //             CliState::Running(running)
    //         }
    //         _ => self,
    //     };
    //     self
    // }
}


#[cfg(test)]
mod tests {
    use crate::{cli::test_mock::_stdin_mock_with_inputted_text, scrape::scrape::{generate_selector, get_html}};

    use super::*;
    use anyhow::{anyhow, Result};

    #[test]
    fn test_constructor_return_start() -> Result<()> {
        let stdin_mock = _stdin_mock_with_inputted_text("");
        let state_machine = CliStateMachine::new(Box::new(stdin_mock));  
        
        match state_machine {
            CliStateMachine::Waiting(_) => {
                Ok(())
            }
            _ => {
                Err(anyhow!("state「Waiting」を期待しましたが違う state で実行されました"))
            }
        }
    }

    #[test]
    fn test_change_running_state_from_waiting() -> Result<()> {
        let stdin_mock = _stdin_mock_with_inputted_text("");
        let state_machine = CliStateMachine::new(Box::new(stdin_mock));

        match state_machine.execute() {
            CliStateMachine::Running(_) => {
                Ok(())
            }
            _ => {
                Err(anyhow!("state「Waiting」を期待しましたが違う state で実行されました"))
            }
        }
    }

    #[test]
    fn test_change_waiting_state_from_running() -> Result<()> {
        let stdin_mock = _stdin_mock_with_inputted_text("");
        let waiting = CliStateMachine::new(Box::new(stdin_mock));
        let running = waiting.execute();

        match running.execute() {
            CliStateMachine::Waiting(_) => {
                Ok(())
            }
            _ => {
                Err(anyhow!("state「Waiting」を期待しましたが違う state で実行されました"))
            }
        }
    }

    // #[tokio::test]
    // async fn test_open_stdin_called_execute_from_waiting() -> Result<()> {
    //     let stdin_mock = stdin_mock_with_inputted_text("テスト入力");
    //     let state = CliState::new(stdin_mock);

    //     if let CliState::Waiting(waiting) = state.execute().await {
    //         assert_eq!(waiting.input, "テスト入力");

    //         Ok(())
    //     } else {
    //         Err(anyhow!("state「Waiting」を期待しましたが違う state で実行されました"))
    //     }
    // }

    // // running 状態で execute を呼び出すと、スクレイピング処理が実行される
    // #[tokio::test]
    // async fn test_scrape_called_execute_from_running() -> Result<()> {
    //     let stdin_mock = stdin_mock_with_inputted_text("テスト入力");
    //     let waiting_state: CliState<std::io::Cursor<&str>> = CliState::new(stdin_mock);
    //     let running_state = waiting_state.execute().await.update();

    //     match running_state.execute().await {
    //         CliState::Running(running) => {
    //             assert_eq!(running.input, "Example Domain");

    //             Ok(())
    //         }
    //         _ => {
    //             Err(anyhow!("state「Running」を期待しましたが違う state で実行されました"))
    //         }
    //     }
    // }
    
}