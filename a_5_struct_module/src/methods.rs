use crate::structs::Rectangle;

// impl 안에 구현된 함수는 연관 함수(associated function) 이라고 함
impl Rectangle {
    pub fn area(&self) -> u32 {
        // &self == `self: &Self`
        self.width * self.height
    }

    // 필드 이름과 중복 가능
    pub fn width(&self) -> bool {
        self.width > 0
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width() > other.width() && self.height > other.height
    }
}

// 여러개의 impl 블록을 가질 수 있음
impl Rectangle {
    // 생성자
    pub fn square(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}
