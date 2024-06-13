use crate::scrape::scrape::fetch_inner_html_into_vec;
use super::{waiting::Waiting, CliState, Transitioning};

pub struct Running {}
impl CliState<Running> {
    pub fn update(self) -> Transitioning {
        // input が "exit" の場合は Transition::Exit を返す
        if self.input.trim() == "exit" {
            return Transitioning::Exit(
                CliState {
                    _state: super::exit::Exit{},
                    input: self.input,
                    html: self.html,
                    stdin: self.stdin
                }
            )
        }
        Transitioning::Waiting(
            CliState {
                _state: Waiting{},
                input:String::new(),
                html: self.html,
                stdin: self.stdin
            }
        )
        
    }

    // TODO エラーハンドリングが適当なので何とかする
    pub async fn execute(&mut self){
        match &mut self.input {
            // "scrape" と入力されていたら、html を取得する
            input if input.trim() == "scrape" => {
                self.html = fetch_inner_html_into_vec("https://example.com/","h1").await.expect("HTML の取得に失敗しました。");
            },
            // "exit" と入力されていたら、終了する
            input if input.trim() == "exit" => {
                println!("終了します。");
            },
            // それ以外の入力は help を表示
            _ => {
                println!("scrape と入力すると HTML を取得できます。")
            }
        }
    }
}