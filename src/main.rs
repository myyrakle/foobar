use std::path::PathBuf;

mod utils;

use utils::get_input;

#[tokio::main]
async fn main() {
    let host = "http://viewer.nl.go.kr:8080/nlmivs/view_image.jsp";

    println!("enter cno:");
    let cno = get_input();

    let mut page = 1;

    loop {
        utils::download("http://viewer.nl.go.kr:8080/nlmivs/view_image.jsp?cno=KOL000022759&vol=1&page=1&twoThreeYn=N".into(),PathBuf::from("foo.png") ).await;
    }

    println!("Hello, world!");
}
