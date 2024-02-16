mod garden;

use std::collections::HashSet as Set; // as keyword 로 alias 지정 가능
pub use crate::garden::vegetables::Asparagus;
use std::collections::*; // glob 연산자를 통해 모든 공개 아이템 가져올 수 있음


fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);

    let mut hashSet = Set::new();
    hashSet.insert(1);
    hashSet.insert(1);

    println!("{:?}", hashSet);
}