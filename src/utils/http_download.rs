use std::path::PathBuf;

use reqwest::header;
use reqwest::ClientBuilder;

pub async fn download(url: String, store_path: PathBuf) {
    let mut headers = header::HeaderMap::new();

    headers.insert("Accept", header::HeaderValue::from_static(r#"text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9"#));
    headers.insert(
        "Accept-Encoding",
        header::HeaderValue::from_static(r#"gzip, deflate"#),
    );
    headers.insert(
        "Accept-Language",
        header::HeaderValue::from_static(r#"ko-KR,ko;q=0.9,en-US;q=0.8,en;q=0.7"#),
    );
    headers.insert(
        "Cache-Control",
        header::HeaderValue::from_static(r#"max-age=0"#),
    );
    headers.insert(
        "Connection",
        header::HeaderValue::from_static(r#"keep-alive"#),
    );
    headers.insert("Cookie", header::HeaderValue::from_static(r#"PCID=91441167-fa7c-7d4b-3d3e-a99fd7c50d7b-1671971267194; _ga=GA1.3.1419761206.1671971267; _gid=GA1.3.1283117326.1671971267; WMONID=GOxMU6GMLr6; JSESSIONID="Kiytrx1Q5YjLRFpS3bXV-x5rWDJsgqPS5o_9LOBT.VWWAS2:tv-4""#));
    headers.insert(
        "Host",
        header::HeaderValue::from_static(r#"viewer.nl.go.kr:8080"#),
    );
    headers.insert(
        "Upgrade-Insecure-Requests",
        header::HeaderValue::from_static(r#"1"#),
    );
    headers.insert("User-Agent", header::HeaderValue::from_static(r#"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36"#));

    let client = ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap();

    if let Ok(data) = client.get(url).send().await {
        println!("foo: {:?}", data);
        println!("bar: {:?}", data.content_length());
        if let Ok(data) = data.bytes().await {
            if let Err(error) = tokio::fs::write(store_path, data).await {
                println!("error{:?}", error);
            }
        }
    }
}
