/// 표준 입력에서 문자열 한줄을 읽어옵니다.
pub fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_owned()
}
