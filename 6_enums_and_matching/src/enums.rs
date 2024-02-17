// enum IpAddrKind {
//     V4,
//     V6,
// }

pub enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Struct 와 비슷하지만 모든 배리언트가 Message 타입으로 묶임
pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(Some),
}
