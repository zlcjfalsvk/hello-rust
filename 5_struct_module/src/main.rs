use crate::structs::{AlwaysEqual, Color, Point, Rectangle, User};

mod methods;
mod structs;

fn main() {
    let mut user1 = build_user(String::from("cheolcheol@gmail.com"), "cheolcheol");
    user1.active = false;
    // println!("{:?}", user1); // struct 의 값을 print 하기 위해 `:?` 사용과 strut 에 `#[derive(Debug)]` 추가가 필요함

    // let user2 = User {
    //     email: String::from("cheolcheol2@gmail.com"),
    //     ..user1
    // };

    // clone 하기 위해 struct 에 #[derive(Clone)] 추가 함
    let user2 = Clone::clone(&user1);
    user1.active = true;

    println!("user1: {:?}", user1);

    // beauty format
    println!("user2: {:#?}", user2);

    // tuple structs , black 과 origin 은 다른 Type
    let black = Color(0, 1, 2);
    // let origin = Point(0, 0, 0);
    println!("{:?}", black.2);

    // unit-like structs
    // let subject = AlwaysEqual;

    let rectangle = Rectangle {
        width: dbg!(10 * 2),
        height: 300,
    };
    // println!("rectangle area is {}", ares(&rectangle));
    // dbg!(ares(&rectangle)); // 함수 선언 방식
    dbg!(rectangle.area()); // 메서드 선언 방식

    if rectangle.width() {
        println!(
            "The rectangle has a nonzero width; it is {}",
            rectangle.width
        );
    }

    let new_rectangle = Rectangle::square(10, 100);
    println!("new_rectangle area is {:?}", new_rectangle.area());
}

fn build_user(email: String, username: &str) -> User {
    User {
        active: true,
        username: username.to_string(),
        email, // JS 처럼 property 와 param name 이 같은 경우 다음과 같이 사용 가능
        sign_in_count: 1,
    }
}

fn ares(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
