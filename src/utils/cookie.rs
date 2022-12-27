use std::time::Duration;

use reqwest::{header, ClientBuilder, StatusCode};

pub async fn get_jsesson_id(cno: String) -> String {
    let mut headers = header::HeaderMap::new();

    headers.insert("Accept", header::HeaderValue::from_static(r#"text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9"#));
    headers.insert(
        "Accept-Encoding",
        header::HeaderValue::from_static(r#"gzip, deflate, br"#),
    );
    headers.insert(
        "Accept-Language",
        header::HeaderValue::from_static(r#"ko-KR,ko;q=0.9"#),
    );
    headers.insert(
        "Connection",
        header::HeaderValue::from_static(r#"keep-alive"#),
    );
    headers.insert(
        "Host",
        header::HeaderValue::from_static(r#"viewer.nl.go.kr"#),
    );
    headers.insert(
        "sec-ch-ua",
        header::HeaderValue::from_static(
            r#""Not?A_Brand";v="8", "Chromium";v="108", "Google Chrome";v="108""#,
        ),
    );
    headers.insert(
        "sec-ch-ua-mobile",
        header::HeaderValue::from_static(r#"?0"#),
    );
    headers.insert(
        "sec-ch-ua-platform",
        header::HeaderValue::from_static(r#""Windows""#),
    );
    headers.insert(
        "Sec-Fetch-Dest",
        header::HeaderValue::from_static(r#"document"#),
    );
    headers.insert(
        "Sec-Fetch-Mode",
        header::HeaderValue::from_static(r#"navigate"#),
    );
    headers.insert(
        "Sec-Fetch-Site",
        header::HeaderValue::from_static(r#"none"#),
    );
    headers.insert("Sec-Fetch-User", header::HeaderValue::from_static(r#"?1"#));
    headers.insert(
        "Upgrade-Insecure-Requests",
        header::HeaderValue::from_static(r#"1"#),
    );
    headers.insert("User-Agent", header::HeaderValue::from_static(r#"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36"#));

    let client = ClientBuilder::new()
        .default_headers(headers)
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(5))
        .connect_timeout(Duration::from_secs(2))
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36")
        .build()
        .unwrap();

    let url = format!(
        "https://viewer.nl.go.kr/nlmivs/viewWonmun_js.jsp?cno={}",
        cno
    );

    let response = client
        .get(url)
        .send()
        .await
        .expect("쿠기 가져오기 실패!! 사이트에 문제가 있습니다.");

    println!("response: {:?}", response);

    let jsession_id = response
        .headers()
        .into_iter()
        .find(|(key, value)| {
            key.as_str() == "set-cookie" && value.to_str().unwrap_or("").starts_with("JSESSIONID=")
        })
        .expect("no JSESSIONID")
        .1
        .to_str()
        .expect("invalid JSESSIONID")
        .to_owned()
        .split(";")
        .into_iter()
        .next()
        .unwrap()
        .to_owned();

    let wmonid = response
        .headers()
        .into_iter()
        .find(|(key, value)| {
            key.as_str() == "set-cookie" && value.to_str().unwrap_or("").starts_with("WMONID=")
        })
        .expect("no wmonid")
        .1
        .to_str()
        .expect("invalid wmonid")
        .to_owned()
        .split(";")
        .into_iter()
        .next()
        .unwrap()
        .to_owned();

    println!("JSESSION_ID: [{}]", jsession_id);
    println!("wmonid: [{}]", wmonid);

    println!("bar: {:?}", response.text().await.unwrap());

    let jsession = format!("{}; {}", jsession_id, wmonid);

    register_jsession(cno, jsession.clone()).await;

    jsession
}

pub async fn register_jsession(cno: String, cookie: String) {
    let mut headers = header::HeaderMap::new();

    let referal_url = format!(
        "https://viewer.nl.go.kr/nlmivs/viewWonmun_js.jsp?cno={}",
        cno
    );

    headers.insert("Accept", header::HeaderValue::from_static(r#"text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9"#));
    headers.insert(
        "Accept-Encoding",
        header::HeaderValue::from_static(r#"gzip, deflate, br"#),
    );
    headers.insert(
        "Accept-Language",
        header::HeaderValue::from_static(r#"ko-KR,ko;q=0.9"#),
    );
    headers.insert(
        "Connection",
        header::HeaderValue::from_static(r#"keep-alive"#),
    );
    headers.insert(
        "Content-Type",
        header::HeaderValue::from_static(r#"application/x-www-form-urlencoded"#),
    );
    headers.insert(
        "Cookie",
        header::HeaderValue::from_str(cookie.as_str()).unwrap(),
    );
    headers.insert(
        "Host",
        header::HeaderValue::from_static(r#"viewer.nl.go.kr"#),
    );
    headers.insert(
        "Origin",
        header::HeaderValue::from_static(r#"https://viewer.nl.go.kr"#),
    );
    headers.insert(
        "Referer",
        header::HeaderValue::from_str(referal_url.as_str()).expect("invalid referal url"),
    );
    headers.insert(
        "sec-ch-ua",
        header::HeaderValue::from_static(
            r#""Not?A_Brand";v="8", "Chromium";v="108", "Google Chrome";v="108""#,
        ),
    );
    headers.insert(
        "sec-ch-ua-mobile",
        header::HeaderValue::from_static(r#"?0"#),
    );
    headers.insert(
        "sec-ch-ua-platform",
        header::HeaderValue::from_static(r#""Windows""#),
    );
    headers.insert(
        "Sec-Fetch-Dest",
        header::HeaderValue::from_static(r#"document"#),
    );
    headers.insert(
        "Sec-Fetch-Mode",
        header::HeaderValue::from_static(r#"navigate"#),
    );
    headers.insert(
        "Sec-Fetch-Site",
        header::HeaderValue::from_static(r#"same-origin"#),
    );
    headers.insert(
        "Upgrade-Insecure-Requests",
        header::HeaderValue::from_static(r#"1"#),
    );
    headers.insert("User-Agent", header::HeaderValue::from_static(r#"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36"#));

    let request_body = format!(
        r#"cno={}&mad=&lip=49.142.71.68&sysid=&v_db=&v_mode=&v_doc_no=&v_doc_id=&sip=49.142.71.68&f=&card_class=&relate_view=N"#,
        cno
    );

    let content_length = request_body.len().to_string();

    headers.insert(
        "Content-Length",
        header::HeaderValue::from_str(content_length.as_str()).expect(""),
    );

    let client = ClientBuilder::new()
        .default_headers(headers)
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36")
        .build()
        .unwrap();

    // KOL000022759

    println!("request_body: {}", request_body);

    let response = client
        .post("https://viewer.nl.go.kr/main.wviewer")
        .body(request_body)
        .send()
        .await
        .expect("jsession 설정 실패");

    println!("foo: {:?}", response.text().await.unwrap());

    // if response.status() != StatusCode::OK {
    //     panic!("jsession 설정 실패: {}", response.status());
    // }
}

/*
`function() {\r\n        var result = "";\r\n\r\n        if(typeof window.crypto != 'undefined' && typeof window.crypto.getRandomValues != 'undefined') {\r\n            var buf = new Uint16Array(8);\r\n            window.crypto.getRandomValues(buf);\r\n            var S4 = function (num) {\r\n                var ret = num.toString(16);\r\n                while (ret.length < 4) {\r\n                    ret = "0" + ret;\r\n                }\r\n                return ret;\r\n            };\r\n\r\n            result = (S4(buf[0]) + S4(buf[1]) + "-" + S4(buf[2]) + "-"\r\n            + S4(buf[3]) + "-" + S4(buf[4]) + "-" + S4(buf[5])\r\n            + S4(buf[6]) + S4(buf[7]));\r\n        }\r\n        else {\r\n            result = 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g,\r\n                function (c) {\r\n                    var r = Math.random() * 16 | 0, v = c == 'x' ? r\r\n                        : (r & 0x3 | 0x8);\r\n                    return v.toString(16);\r\n                });\r\n        }\r\n\r\n        return result + '-' + new Date().getTime();\r\n    }`
*/
// pub async fn get_pcid() -> String {
//     "PCID=d24e082c-7e62-69a2-e70b-d0ad92ff154b-1672115526388".to_owned()
// }

// Cookie: PCID=d24e082c-7e62-69a2-e70b-d0ad92ff154b-1672115526389; _ga=GA1.3.1956402345.1672115527; _gid=GA1.3.1508585817.1672115527; WMONID=jzF-pJ8hXFL; JSESSIONID="whuFhCLPRpnGoNSrsCABFMChRfGU9ZS0oEK2IAoz.VWWAS1:tv-3"
// https://www.nl.go.kr/NL/contents/search.do?srchTarget=total&pageNum=1&pageSize=10&kwd=%E5%B0%91%E5%BE%AE%E9%80%9A%E9%91%91%E7%AF%80%E8%A6%81#viewKey=CNTS-00047689282&viewType=C&category=%EA%B3%A0%EB%AC%B8%ED%97%8C&pageIdx=1&jourId=
// https://viewer.nl.go.kr/main.wviewer
// CNTS-00047689282

pub async fn get_headers(cno: String) -> header::HeaderMap {
    let mut headers = header::HeaderMap::new();

    let jsession_id = get_jsesson_id(cno).await;

    let cookie = format!(r#"{}"#, jsession_id);

    headers.insert(
        "Accept",
        header::HeaderValue::from_static(
            r#"image/avif,image/webp,image/apng,image/svg+xml,image/*,*/*;q=0.8"#,
        ),
    );
    headers.insert(
        "Accept-Encoding",
        header::HeaderValue::from_static(r#"gzip, deflate, br"#),
    );
    headers.insert(
        "Accept-Language",
        header::HeaderValue::from_static(r#"ko-KR,ko;q=0.9"#),
    );
    headers.insert(
        "Connection",
        header::HeaderValue::from_static(r#"keep-alive"#),
    );
    headers.insert(
        "Cookie",
        header::HeaderValue::from_str(cookie.as_str()).unwrap(),
    );
    headers.insert(
        "Host",
        header::HeaderValue::from_static(r#"viewer.nl.go.kr"#),
    );
    headers.insert(
        "Referer",
        header::HeaderValue::from_static(r#"https://viewer.nl.go.kr/main.wviewer"#),
    );
    headers.insert(
        "sec-ch-ua",
        header::HeaderValue::from_static(
            r#""Not?A_Brand";v="8", "Chromium";v="108", "Google Chrome";v="108""#,
        ),
    );
    headers.insert(
        "sec-ch-ua-mobile",
        header::HeaderValue::from_static(r#"?0"#),
    );
    headers.insert(
        "sec-ch-ua-platform",
        header::HeaderValue::from_static(r#""Windows""#),
    );
    headers.insert(
        "Sec-Fetch-Dest",
        header::HeaderValue::from_static(r#"image"#),
    );
    headers.insert(
        "Sec-Fetch-Mode",
        header::HeaderValue::from_static(r#"no-cors"#),
    );
    headers.insert(
        "Sec-Fetch-Site",
        header::HeaderValue::from_static(r#"same-origin"#),
    );
    headers.insert("User-Agent", header::HeaderValue::from_static(r#"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36"#));

    headers
}
