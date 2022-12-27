use std::path::PathBuf;

use reqwest::header;
use reqwest::ClientBuilder;
use reqwest::StatusCode;

pub async fn download(url: String, headers: header::HeaderMap, store_path: PathBuf) -> StatusCode {
    let client = ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap();

    if let Ok(data) = client.get(url).send().await {
        let status = data.status().clone();

        if let Ok(data) = data.bytes().await {
            if let Err(error) = tokio::fs::write(store_path, data).await {
                println!("error{:?}", error);
            }
        }

        return status;
    } else {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }
}
