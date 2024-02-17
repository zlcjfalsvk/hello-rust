fn main() {
    let mut s1 = gives_ownership();         // gives_ownership이 자신의 반환 값을 s1로
    // 이동시킵니다
    s1.push_str(" yours");
    println!("s1: {s1}");

    let s2 = String::from("hello");     // s2가 스코프 안으로 들어옵니다
    let s3 = takes_and_gives_back(s2);     // s2는 takes_and_gives_back로 이동되는데,
                                                  // 이 함수 또한 자신의 반환 값을 s3로
                                                  // 이동시킵니다

    // main 밖에서 s3가 스코프 밖으로 벗어나면서 버려집니다. s2는 이동되어서 아무 일도
    // 일어나지 않습니다. s1은 스코프 밖으로 벗어나고 버려집니다.

    // s2.push_str(" rust");
    // 위의 코드 주석 해제시 error, takes_and_gives_back(s2.clone()); 깊은 복사 해야 함
    println!("s3 {s3}");

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");
    change(&mut s);


    let mut s = String::from("hello");
    let r1 = &s;
    // let r2 = &mut s; // race condition 발생
    // println!("{}, {}", r1, r2);
    // 두개 이상의 포인터 참조 중 쓰기 작업 실행 시 발생
    // 위의 해결 방법 중 하나로 shadowing 사용 가능
    {
        let r2 = &mut s; // race condition 발생
    }

    let mut s = String::from("Hello World"); // 5
    let world = first_word(&s);
    println!("world: {world}");

    // slice
    let hello = &s[0..5]; // == &s[..5];
    let world = &s[6..s.len()]; // ==&s[6..];
    // let helloWorld = &s[..]; // == &s[0..s.len()];




    // deref coercions (역참조 강제) 를 활용한 param 에 &String, &str 모두 받기 -------------------
    let my_string = String::from("hello world");
    // `first_word`는 `String`의 일부 혹은 전체 슬라이스에 대해 작동합니다
    let word = first_word_for_deref(&my_string[0..6]);
    let word = first_word_for_deref(&my_string[..]);
    // 또한 `first_word`는 `String`의 전체 슬라이스와 동일한 `String`의
    // 참조자에 대해서도 작동합니다
    let word = first_word_for_deref(&my_string);

    let my_string_literal = "hello world";

    // `first_word`는 문자열 리터럴의 일부 혹은 전체 슬라이스에 대해 작동합니다
    let word = first_word_for_deref(&my_string_literal[0..6]);
    let word = first_word_for_deref(&my_string_literal[..]);

    // 문자열 리터럴은 *곧* 문자열 슬라이스이므로,
    // 아래의 코드도 슬라이스 문법 없이 작동합니다!
    let word = first_word_for_deref(my_string_literal);

    // deref coercions (역참조 강제) 를 활용한 param 에 &String, &str 모두 받기 -------------------

}

fn gives_ownership() -> String {             // gives_ownership은 자신의 반환 값을
                                             // 자신의 호출자 함수로 이동시킬
                                             // 것입니다
    let some_string = String::from("yours"); // some_string이 스코프 안으로 들어옵니다
    some_string                              // some_string이 반환되고
                                             // 호출자 함수 쪽으로
                                             // 이동합니다
}

// 이 함수는 String을 취하고 같은 것을 반환합니다
fn takes_and_gives_back(a_string: String) -> String { // a_string이 스코프 안으로
                                                      // 들어옵니다
    a_string  // a_string이 반환되고 호출자 함수 쪽으로 이동합니다
}

// referencing => & 기호 사용
// dereferencing => * 기호 사용
fn calculate_length(s: &String) -> (usize) {
    let length = s.len(); // len()은 String의 길이를 반환합니다
    length
}   // 여기서 s가 스코프 밖으로 벗어납니다. 하지만 참조하는 것을 소유하고 있진 않으므로,
    // 버려지지는 않습니다.


// mutable reference
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//
//     s.len()
// }
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_for_deref(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
