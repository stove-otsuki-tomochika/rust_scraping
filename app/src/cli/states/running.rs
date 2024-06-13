use super::{waiting::Waiting, CliState};

pub struct Running {}
impl CliState<Running> {
    pub fn update(self) -> CliState<Waiting> {
        CliState {_state: Waiting{},input:String::new() , stdin: self.stdin}
    }

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