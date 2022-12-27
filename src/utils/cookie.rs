use reqwest::{header, ClientBuilder};

pub async fn get_jsesson_id(cno: String, pcid: String) -> String {
    let mut headers = header::HeaderMap::new();

    headers.insert(
        "Cookie",
        header::HeaderValue::from_str(pcid.as_str()).unwrap(),
    );

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

    let url = format!(
        "https://viewer.nl.go.kr/nlmivs/viewWonmun_js.jsp?cno={}",
        cno
    );

    let response = client
        .get(url)
        .send()
        .await
        .expect("쿠기 가져오기 실패!! 사이트에 문제가 있습니다.");

    let jsession_id = response.headers().into_iter().find(|(key, value)| {
        key.as_str() == "set-cookie" && value.to_str().unwrap_or("").starts_with("JSESSIONID=")
    });

    let wmonid = response.headers().into_iter().find(|(key, value)| {
        println!("{:?}", value);
        key.as_str() == "set-cookie" && value.to_str().unwrap_or("").starts_with("WMONID=")
    });

    let jsession_id = jsession_id
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

    let wmonid = wmonid
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

    println!("JSESSION_ID {}", jsession_id);

    format!(
        "{}",
        r#"JSESSIONID="whuFhCLPRpnGoNSrsCABFMChRfGU9ZS0oEK2IAoz.VWWAS1:tv-3""#,
    )
}

/*
`function() {\r\n        var result = "";\r\n\r\n        if(typeof window.crypto != 'undefined' && typeof window.crypto.getRandomValues != 'undefined') {\r\n            var buf = new Uint16Array(8);\r\n            window.crypto.getRandomValues(buf);\r\n            var S4 = function (num) {\r\n                var ret = num.toString(16);\r\n                while (ret.length < 4) {\r\n                    ret = "0" + ret;\r\n                }\r\n                return ret;\r\n            };\r\n\r\n            result = (S4(buf[0]) + S4(buf[1]) + "-" + S4(buf[2]) + "-"\r\n            + S4(buf[3]) + "-" + S4(buf[4]) + "-" + S4(buf[5])\r\n            + S4(buf[6]) + S4(buf[7]));\r\n        }\r\n        else {\r\n            result = 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g,\r\n                function (c) {\r\n                    var r = Math.random() * 16 | 0, v = c == 'x' ? r\r\n                        : (r & 0x3 | 0x8);\r\n                    return v.toString(16);\r\n                });\r\n        }\r\n\r\n        return result + '-' + new Date().getTime();\r\n    }`
*/
pub async fn get_pcid() -> String {
    "PCID=d24e082c-7e62-69a2-e70b-d0ad92ff154b-1672115526388".to_owned()
}

// Cookie: PCID=d24e082c-7e62-69a2-e70b-d0ad92ff154b-1672115526389; _ga=GA1.3.1956402345.1672115527; _gid=GA1.3.1508585817.1672115527; WMONID=jzF-pJ8hXFL; JSESSIONID="whuFhCLPRpnGoNSrsCABFMChRfGU9ZS0oEK2IAoz.VWWAS1:tv-3"
// https://www.nl.go.kr/NL/contents/search.do?srchTarget=total&pageNum=1&pageSize=10&kwd=%E5%B0%91%E5%BE%AE%E9%80%9A%E9%91%91%E7%AF%80%E8%A6%81#viewKey=CNTS-00047689282&viewType=C&category=%EA%B3%A0%EB%AC%B8%ED%97%8C&pageIdx=1&jourId=
// https://viewer.nl.go.kr/main.wviewer
// CNTS-00047689282

pub async fn get_headers(cno: String) -> header::HeaderMap {
    let mut headers = header::HeaderMap::new();

    let pcid = get_pcid().await;

    let jsession_id = get_jsesson_id(cno, pcid.clone()).await;

    let cookie = format!(
        r#"{}; _ga=GA1.3.1419761206.1671971267; _gid=GA1.3.1283117326.1671971267; WMONID=GOxMU6GMLr6; {}"#,
        pcid, jsession_id
    );

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
    headers.insert(
        "Cookie",
        header::HeaderValue::from_str(cookie.as_str()).unwrap(),
    );
    headers.insert(
        "Host",
        header::HeaderValue::from_static(r#"viewer.nl.go.kr:8080"#),
    );
    headers.insert(
        "Upgrade-Insecure-Requests",
        header::HeaderValue::from_static(r#"1"#),
    );
    headers.insert("User-Agent", header::HeaderValue::from_static(r#"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36"#));

    headers
}
