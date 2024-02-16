mod structs;

use crate::enums::{IpAddrKind, Message};
use std::fmt::Octal;
use libs::type_of;
use structs::IpAddr;


mod enums;
mod impls;

fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddr {
        kind: IpAddrKind::V6(String::from("::1")),
        address: String::from("::1"),
    };

    let m = Message::Write(String::from("hello"));
    m.call();

    // rust 는 null 개념 대신 Option 이라는 enum 사용
    let some_number = Some(5);
    // let some_char = Some('e');
    let absent_number: Option<i32> = None;
    let num = some_number.unwrap();

    println!("Some to i32: {:?} to {}", some_number, num);
    println!("num typeof is {}", type_of(&num));
    println!("None value {:?}", absent_number);
}

fn rout(ip_addr_kind: IpAddrKind) {}
