use std::fs::File;
use std::io::{ErrorKind, Read};
use std::{fs, io};

fn main() {
    // panic!("crash and burn"); // panic 강제 발생 시키기

    // let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match greeting_file_result {    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error);
    //         }
    //     },
    // };

    // unwrap_or_else 및 클로저 사용, |error| 이 부분이 클로저를 정의 한다
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error|
        // match error.kind() {
        //     ErrorKind::NotFound => match File::create("hello.txt") {
        //         Ok(fc) => fc,
        //         Err(e) => panic!("Problem creating the file: {:?}", e),
        //     },
        //     other_error => {
        //         panic!("Problem opening the file: {:?}", other_error);
        // }}
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    );

    // unwrap 보다는 expect 를 좀 더 사용하자
    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");

    // propagating
    let f = read_username_from_file();
    if let Err(_err) = f {
        panic!("Err")
    }

    // let g = Guess::new(3);

    // Result -> Option 으로 변경
    let x: Result<u32, &str> = Ok(2);
    assert_eq!(x.ok(), Some(2));

    let x: Result<u32, &str> = Err("Nothing here");
    assert_eq!(x.ok(), None);
}

// propagating, 함수를 호출하는 쪽에서 에러 핸들링을 할 수 있게 함
// fn read_username_from_file() -> Result<String, io::Error> {
//     let username_file_result = File::open("hello.txt");
//
//     let mut username_file = match username_file_result {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };
//
//     let mut username = String::new();
//
//     // match 를 이용하여 호출 쪽으로 err 를 반환 함
//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => Err(e),
//     }
// }

// 위의 코드의 shortcut
// ? 연산자는 위 코드와 다르게 from 함수를 거친다
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)

    // => 함수 제공
    // fs::read_to_string("hello.text");
}
