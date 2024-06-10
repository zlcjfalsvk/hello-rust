use regex::Regex;

fn main() {
    let str = String::from("a->b");

    // 이 때 r 을 붙이는 이유는 raw string literal 을 사용하기 위함
    // r 을 사용하면 escape(\) 등의 특수문자를 사용하지 않고 그대로 문자열에 포함 가능
    // 밑은 다음과 같음: Regex::new("\\+>")
    let is_match = Regex::new(r"\+>").unwrap().is_match(str.as_str());

    println!("{is_match}"); // false
}
