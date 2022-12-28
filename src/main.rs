use epoch_timestamp::Epoch;
use reqwest::StatusCode;
use std::path::PathBuf;

mod utils;

use utils::get_input;

#[tokio::main]
async fn main() {
    let host = "http://viewer.nl.go.kr/nlmivs/view_image.jsp";

    println!("cno가 뭐죠?:");
    let cno = get_input();

    let jsession_id = utils::get_jsesson_id(cno.clone()).await;
    utils::register_jsession(cno.clone(), jsession_id.clone()).await;
    let (title, total_page) = utils::register_jsession2(cno.clone(), jsession_id.clone()).await;

    let headers = utils::get_headers(jsession_id).await;

    println!("# INFO: 전체 페이지 {}", total_page);

    // println!("시작 페이지:");
    // let start_page: i32 = get_input().parse().expect("유효한 숫자가 아닙니다.");

    // println!("종료 페이지:");
    // let end_page: i32 = get_input().parse().expect("유효한 숫자가 아닙니다.");

    let end_page = total_page;

    let mut page = 1; //start_page;

    let directory_path = PathBuf::new().join(format!("{}-{}", title, Epoch::now().to_string()));

    tokio::fs::create_dir(&directory_path).await.unwrap();
    println!(">>> 폴더 생성: {}", directory_path.display());

    'OUTER: loop {
        if page > end_page {
            println!(">>> 중단");
            break;
        }

        let file_path = directory_path.join(format!("{}.png", page));

        // 재시도 루프
        'INNER: loop {
            let status = utils::download(
                format!("{}?cno={}&vol=1&page={}&twoThreeYn=N", host, cno, page).into(),
                headers.clone(),
                file_path.clone(),
            )
            .await;

            match status {
                StatusCode::NOT_FOUND => {
                    println!(">>> 중단");
                    break 'OUTER;
                }
                StatusCode::OK => {
                    println!(">>> 파일 생성: {}", file_path.display());

                    page += 1;

                    std::thread::sleep(std::time::Duration::from_millis(500));
                    break 'INNER;
                }
                _ => {
                    println!("! 오류 발생. 재시도...");
                    continue;
                }
            }
        }
    }

    println!("처리완료");
    get_input();
}
