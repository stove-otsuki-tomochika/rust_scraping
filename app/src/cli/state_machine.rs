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

    pub async fn execute(self) -> Self {
        match self {
            CliStateMachine::Waiting(mut waiting) => {
                waiting.execute();
                CliStateMachine::Running(waiting.update())
            },
            CliStateMachine::Running(mut running) => {
                running.execute().await;
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
    use crate::{cli::{states::running, test_mock::_stdin_mock_with_inputted_text}, scrape::scrape::{generate_selector, get_html}};

    use super::*;
    use anyhow::{anyhow, Result};

    // CliStateMachine のコンストラクタを呼び出すと、Waiting 状態から始まる
    #[test]
    fn test_constructor_return_waiting_state() -> Result<()> {
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

    // Waiting 状態で execute を呼び出すと標準入力からの入力を受け付け、
    // それを保持して Running 状態に遷移する
    #[tokio::test]
    async fn test_open_stdin_and_change_running_state_from_waiting() -> Result<()> {
        let stdin_mock = _stdin_mock_with_inputted_text("テスト入力");
        let state_machine = CliStateMachine::new(Box::new(stdin_mock));

        match state_machine.execute().await {
            CliStateMachine::Running(running) => {
                assert_eq!(running.input, "テスト入力");
                Ok(())
            }
            _ => {
                Err(anyhow!("state「Waiting」を期待しましたが違う state で実行されました"))
            }
        }
    }

    // Running 状態で execute を呼び出すと、Waiting 状態に遷移する
    #[tokio::test]
    async fn test_change_waiting_state_from_running() -> Result<()> {
        let stdin_mock = _stdin_mock_with_inputted_text("");
        let waiting = CliStateMachine::new(Box::new(stdin_mock));
        let running = waiting.execute().await;

        match running.execute().await {
            CliStateMachine::Waiting(_) => {
                Ok(())
            }
            _ => {
                Err(anyhow!("state「Waiting」を期待しましたが違う state で実行されました"))
            }
        }
    }

    // running 状態で execute を呼び出すと、スクレイピング処理が実行される
    #[tokio::test]
    async fn test_scrape_called_execute_from_running() -> Result<()> {
        let stdin_mock = _stdin_mock_with_inputted_text("テスト入力");
        let waiting = CliStateMachine::new(Box::new(stdin_mock));
        let running = waiting.execute().await;

        match running.execute().await {
            CliStateMachine::Waiting(waiting) => {
                assert_eq!(waiting.html[0], "Example Domain");
            }
            _ => {
                return Err(anyhow!("state「Waiting」を期待しましたが違う state で実行されました"));
            }
        }
        Ok(())
    }
}