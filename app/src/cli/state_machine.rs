use std::io::BufRead;


use super::states::{CliState, Transitioning};
use super::states::waiting::Waiting;
use super::states::running::Running;
use super::states::exit::Exit;

pub enum CliStateMachine {
    Waiting(CliState<Waiting>),
    Running(CliState<Running>),
    Exit(CliState<Exit>),
}
impl CliStateMachine {
    pub fn new(stdin: Box<dyn BufRead>) -> Self {
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
                running.update().into()
            },

            // TODO エラーが返るようにする
            _ => self
        }
    }
}

// Transitioning から CliStateMachine に変換する
// Waiting と Exit の2パターンに分岐
impl From<Transitioning> for CliStateMachine {
    fn from(transitioning: Transitioning) -> Self {
        match transitioning {
            Transitioning::Waiting(waiting) => CliStateMachine::Waiting(waiting),
            Transitioning::Exit(exit) => CliStateMachine::Exit(exit),
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::cli::test_mock::_stdin_mock_with_inputted_text;

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

    // input に "exit" がある状態で Running から execute を呼び出すと、Exit 状態に遷移する
    #[tokio::test]
    async fn test_change_exit_from_running_when_input_exit() -> Result<()> {
        let stdin_mock = _stdin_mock_with_inputted_text("exit");
        let waiting = CliStateMachine::new(Box::new(stdin_mock));
        let running = waiting.execute().await;
        let exit = running.execute().await;

        match exit {
            CliStateMachine::Exit(_) => {
                Ok(())
            }
            _ => {
                Err(anyhow!("state「Exit」を期待しましたが違う state で実行されました"))
            }
        }
    }
}