use std::io::BufRead;
use anyhow::{Context,Result};

use crate::scrape::scrape::{generate_selector, get_html};

use super::io::open_stdin;

// pub enum CliState<T:BufRead> {
//     Waiting(Waiting<T>),
//     Running(Running),
//     Exit(Exit),
// }

// impl<T:BufRead> CliState<T> {
//     pub fn new(stdin:T) -> Self {
//         CliState::Waiting(Waiting::new(stdin))
//     }
// }

// pub struct Waiting<T:BufRead> {
//     input: String,
//     stdin: T
// }
// impl<T:BufRead> Waiting<T> {
//     pub fn update(&self) -> CliState<T> {
//         if let "exit" = self.input.as_str() {
//             return CliState::Exit(Exit {input: self.input.clone()})
//         }
//         CliState::Running(Running {input: self.input.clone()})
//     }

//     pub fn input(&mut self, input: &str) -> &mut Waiting<T> {
//         self.input = input.to_string();
//         self
//     }

//     pub fn new(stdin:T) -> Waiting<T> {
//         Waiting {stdin:stdin, input: String::new()}
//     }

//     pub fn execute(&mut self) {
//         let mut input_from_user = String::new();
//         self.stdin
//             .read_line(&mut input_from_user)
//             .expect("入力値が読み取れませんでした。");
//         self.input(input_from_user.trim());
//     }
// }
// struct Running {
//     input: String
// }
// struct Exit {
//     input: String
// }

pub enum CliStateMachine {
    Waiting(CliState<Waiting>),
    Running(CliState<Running>),
    Exit(CliState<Exit>),
}
impl CliStateMachine {
    fn new(stdin: Box<dyn BufRead>) -> Self {
        CliStateMachine::Waiting(CliState::new(Waiting{}, stdin))
    }

    // pub fn update(mut self) -> Self {
    //     self = match self {
    //         CliState::Waiting(waiting) => {
    //             CliState::Running(
    //                 Running {
    //                     input: waiting.input,
    //                     stdin: waiting.stdin
    //                 }
    //             )
    //         }
    //         CliState::Running(running) => {
    //             CliState::Waiting(
    //                 Waiting {
    //                     input: running.input,
    //                     stdin: running.stdin
    //                 }
    //             )
    //         }
    //         CliState::Exit(exit) => {
    //             // TODO エラーにするべき
    //             CliState::Running(
    //                 Running {
    //                     input: exit.input,
    //                     stdin: exit.stdin
    //                 }
    //             )
    //         }
    //     };
    //     self
    // }

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

struct CliState<T> {
    _state:T,
    input: String,
    stdin: Box<dyn BufRead>
}
impl CliState<Waiting> {
    pub fn new(state:Waiting, stdin:Box<dyn BufRead>) -> Self {
        CliState {
            _state: state,
            input: String::new(),
            stdin: stdin
        }
    }
}
impl CliState<Running> {}
impl CliState<Exit> {}

struct Waiting {}
impl Waiting{
    // pub fn execute(&mut self) {
    //     self.stdin
    //     .read_line(&mut self.input)
    //     .expect("入力値が読み取れませんでした。");
    // }
}

struct Running {}
impl Running {
    // // 上流のエラーハンドリングが適当なので何とかする
    // pub async fn execute(&mut self) -> Result<()>{
    //     let fragment = get_html("https://example.com/").await.context("HTML のデータ取得に失敗しました。")?;
    //     let selector = generate_selector("h1").context("h1 タグのセレクタの生成に失敗しました。")?;
    //     let mut html = String::new();

    //     for element in fragment.select(&selector) {
    //         // example.com の h1 タグの中身は "Example Domain" なので、これを検証する
    //         println!("{}", element.inner_html());
    //         html.push_str(&element.inner_html());
    //     }
    //     self.input = html;

    //     Ok(())
    // }
}

struct Exit {}

#[cfg(test)]
mod tests {
    use crate::{cli::test_mock::stdin_mock_with_inputted_text, scrape::scrape::{generate_selector, get_html}};

    use super::*;
    use anyhow::{anyhow, Result};

    #[test]
    fn test_constructor_return_start() -> Result<()> {
        let stdin_mock = stdin_mock_with_inputted_text("");
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

    // #[test]
    // fn test_change_running_state_from_waiting() -> Result<()> {
    //     let stdin_mock = stdin_mock_with_inputted_text("");
    //     let state = CliState::new(stdin_mock).update();

    //     match state {
    //         CliState::Waiting(_) => {
    //             Err(anyhow!("state「Running」を期待しましたが Waiting でした"))
    //         }
    //         CliState::Running(_) => {
    //             Ok(())
    //         }
    //         CliState::Exit(_) => {
    //             Err(anyhow!("state「Running」を期待しましたが Exit でした"))
    //         }
    //     }
    // }

    // #[test]
    // fn test_change_waiting_state_from_running() -> Result<()> {
    //     let stdin_mock = stdin_mock_with_inputted_text("");
    //     let state = CliState::new(stdin_mock);
    //     let waiting = state.update();

    //     match waiting.update() {
    //         CliState::Waiting(_) => {
    //             Ok(())
    //         }
    //         CliState::Running(_) => {
    //             Err(anyhow!("state「Waiting」を期待しましたが Running でした"))
    //         }
    //         CliState::Exit(_) => {
    //             Err(anyhow!("state「Waiting」を期待しましたが Exit でした"))
    //         }
    //     }
    // }

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