use std::cmp::Ordering;

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {:?}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {:?}", result);

    let p = Point { x: 5, y: 10.1 };
    let p2 = Point { x: 6, y: 10.2 };
    let point_list = vec![p, p2];
    let point_list_result = largest(&point_list);

    // println!("p.x = {}", p.x());
    println!("{:?}", point_list_result);
}

// 정렬 가능한 타입에 대해서만 허용
fn largest<T: PartialOrd>(list: &[T]) -> std::option::Option<&T> {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    Some(largest)
}

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

// PartialEq 트레이트 구현
impl<T: PartialEq, U: PartialEq> PartialEq for Point<T, U> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

// PartialOrd 트레이트 구현
impl<T: PartialOrd, U: PartialOrd> PartialOrd for Point<T, U> {
    fn partial_cmp(&self, other: &Self) -> std::option::Option<std::cmp::Ordering> {
        match self.x.partial_cmp(&other.x) {
            Some(Ordering::Equal) => self.y.partial_cmp(&other.y),
            other => other,
        }
    }
}