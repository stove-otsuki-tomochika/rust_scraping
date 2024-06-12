use std::io::BufRead;

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

pub enum CliState<T:BufRead> {
    Waiting(Waiting<T>),
    Running(Running<T>),
    Exit(Exit<T>),
}
impl<T:BufRead> CliState<T> {
    pub fn new(stdin:T) -> Self {
        CliState::Waiting(Waiting{input: String::new(), stdin})
    }
}

trait State {}
struct Waiting<T:BufRead> {
    input: String,
    stdin: T
}
impl<T:BufRead> State for Waiting<T> {}

struct Running<T:BufRead> {
    input: String,
    stdin: T
}
impl<T:BufRead> State for Running<T> {}

struct Exit<T:BufRead> {
    input: String,
    stdin: T
}
impl<T:BufRead> State for Exit<T> {}


#[cfg(test)]
mod tests {
    use crate::cli::test_mock::stdin_mock_with_inputted_text;

    use super::*;
    use anyhow::{anyhow, Result};

    #[test]
    fn test_constructor_return_start() -> Result<()> {
        let stdin_mock = stdin_mock_with_inputted_text("");
        let state = CliState::new(stdin_mock);

        match state {
            CliState::Waiting(waiting) => {
                assert_eq!(waiting.input, "");
                assert_eq!(waiting.stdin, stdin_mock_with_inputted_text(""));
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

    // #[test]
    // fn test_running_state_called_update_from_waiting() -> Result<()> {
    //     let stdin_mock = stdin_mock_with_inputted_text("");
    //     let mut state = CliState::new(stdin_mock);
    //     if let CliState::Waiting(waiting) = &mut state {
    //         state = waiting.update();
    //     }
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
    // fn test_update_input_field_called_input() {
    //     let stdin_mock = stdin_mock_with_inputted_text("");
    //     let mut state = Waiting::new(stdin_mock);
    //     let state = state.input("after");
    //     assert_eq!(state.input, "after");
    // }

    // #[test]
    // fn test_change_exit_state_from_waiting() -> Result<()> {
    //     let stdin_mock = stdin_mock_with_inputted_text("");
    //     let mut state = CliState::new(stdin_mock);
    //     if let CliState::Waiting(waiting) = &mut state {
    //         state = waiting.input("exit").update();
    //     }
    //     match state {
    //         CliState::Waiting(_) => {
    //             Err(anyhow!("state「Exit」を期待しましたが Waiting でした"))
    //         }
    //         CliState::Running(_) => {
    //             Err(anyhow!("state「Exit」を期待しましたが Running でした"))
    //         }
    //         CliState::Exit(_) => {
    //             Ok(())
    //         }
    //     }
    // }

    // #[test]
    // fn test_open_stdin_called_execute_from_waiting() -> Result<()> {
    //     let stdin_mock = stdin_mock_with_inputted_text("テスト入力");
    //     let mut state = CliState::new(stdin_mock);
    //     if let CliState::Waiting(waiting) = &mut state {
    //         waiting.execute();

    //         assert_eq!(waiting.input, "テスト入力");
    //     }
    //     Ok(())
    // }
}