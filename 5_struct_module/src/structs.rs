// 외부속성 선언(outer attribute)
#[derive(Debug, Clone)]
pub struct User {
    pub(crate) active: bool,
    pub(crate) username: String,
    pub(crate) email: String,
    pub(crate) sign_in_count: u64,
}

// tuple structs
#[derive(Debug)]
pub struct Color(pub i32, pub i32, pub i32);
#[derive(Debug)]
pub struct Point(pub i32, pub i32, pub i32);

// unit-like structs
pub struct AlwaysEqual;

#[derive(Debug)]
pub struct Rectangle {
    pub(crate) width: u32,
    pub height: u32
}