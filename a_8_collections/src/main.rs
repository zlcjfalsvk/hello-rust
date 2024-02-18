use std::collections::HashMap;

fn main() {
    // let v: Vec<i32> = Vec::new();
    let mut v = vec!["1", "2", "3"];
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2]; // 3
    let third: Option<&i32> = v.get(2); // 3
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
    // let does_not_exist = &v[100]; // panic 처리 해야 함
    // let does_not_exist = v.get(100); // None

    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6);
    // println!("The first element is: {first}"); // ownership issue

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    } // [150, 82, 107]

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:?}", row);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // String + &str 조합만 가능, 아래의 경우 &String -> &str 로 강제되었음
    let s3 = s1 + &s2; // s1은 여기로 이동되어 더 이상 사용할 수 없음

    let s1 = String::from("Hello, ");
    // let s = s1 + "-" + &s2 + "-" + &s3;
    let s = format!("{s1}-{s2}-{s3}");
    println!("{:?}", s);

    let hello = String::from("Здравствуйте");

    // println!("hello len: {}", hello.len()); // 12가 아닌 24가 나오는 이유는 utf8 로 인코딩된 바이트들의 크기를 의미
    println!("hello len: {}", hello.chars().count());

    let hello = "Здравствуйте";
    let s = &hello[0..4]; // &hello[0..1] 일경우 panic 발생 => utf8 2바이트씩 저장하는데
    println!("{:?}", s); // Зд

    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("{:?}", score);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
    println!("{}", text);
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
