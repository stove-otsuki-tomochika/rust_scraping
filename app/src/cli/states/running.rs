use crate::scrape::scrape::{generate_selector, get_html};
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
        let fragment = get_html("https://example.com/").await.expect("HTML のデータ取得に失敗しました。");
        let selector = generate_selector("h1").expect("h1 タグのセレクタの生成に失敗しました。");
        let mut html = vec![];

        for element in fragment.select(&selector) {
            // example.com の h1 タグの中身は "Example Domain" なので、これを検証する
            println!("{}", element.inner_html());
            html.push(element.inner_html());
        }
        self.html = html;
    }
}