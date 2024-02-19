use std::fmt::Display;
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    // let r;
    // {
    //     let x = 5;
    //     r = &x; // error 발생, 소유권 이전 없이 해당 스코프 안에서 참조를 하고 라이프 타임 이후 사용하려했기 때문에 dangling reference 발생
    // }
    // println!("r: {}", r);

    let mut string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    let mut result = String::from(result);
    result.push_str(" is long");
    string1.push_str("efg");
    println!("{} {} {} ", string1, string2, result);

    let novel = String::from("Call me Ishmael. Some years ago.");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    println!("{:?}", first_sentence);
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{}", i.level());
}

// 함수 시그니처에서 lifetime 명시
// 함수가 반환하는 참조자의 라이프타임은 함수 인수로서 참조된 값들의 라이프타임 중 작은 것과 동일
fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}

// 정적 lifetime 명시
// fn longest(str1: &str, str2: &str) -> &'static str {
//     if str1.len() > str2.len() {
//         str1
//     } else {
//         str2
//     }
// }

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
