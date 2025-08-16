// 제네릭(Generic) 타입 학습
// 제네릭을 사용하면 여러 타입에 대해 동작하는 코드를 작성할 수 있음

use std::cmp::Ordering;

fn main() {
    // 숫자 리스트에서 가장 큰 값 찾기
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {:?}", result);

    // 문자 리스트에서 가장 큰 값 찾기 (알파벳 순서)
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {:?}", result);

    // 제네릭 구조체를 사용한 예제
    let p = Point { x: 5, y: 10.1 };
    let p2 = Point { x: 6, y: 10.2 };
    let point_list = vec![p, p2];
    let point_list_result = largest(&point_list);
    
    // p는 이미 vec!로 소유권이 이동되어 사용 불가
    // println!("p.x = {}", p.x());  // 컴파일 에러
    println!("{:?}", point_list_result);
}

// 제네릭 함수: 여러 타입에 대해 동작하는 함수
// T: PartialOrd - 트레잇 바운드: T는 비교 가능한 타입이어야 함
// &[T] - T 타입의 슬라이스 참조
fn largest<T: PartialOrd>(list: &[T]) -> std::option::Option<&T> {
    let mut largest = &list[0];  // 첫 번째 요소를 가장 큰 값으로 초기화

    for item in list {
        if item > largest {  // PartialOrd 트레잇 덕분에 > 연산자 사용 가능
            largest = item;
        }
    }

    Some(largest)  // Option 타입으로 감싸서 반환
}

// 제네릭 구조체: 서로 다른 타입의 필드를 가질 수 있음
// T, U는 제네릭 타입 파라미터
#[derive(Debug)]  // Debug 트레이트 자동 구현으로 {:?} 형식 출력 가능
struct Point<T, U> {
    x: T,  // x는 T 타입
    y: U,  // y는 U 타입 (같을 수도, 다를 수도 있음)
}

// PartialEq 트레이트 구현: 두 Point를 == 연산자로 비교 가능하게 함
// T와 U 모두 PartialEq를 구현해야 함 (트레잇 바운드)
impl<T: PartialEq, U: PartialEq> PartialEq for Point<T, U> {
    fn eq(&self, other: &Self) -> bool {
        // x와 y 모두 같을 때 true 반환
        self.x == other.x && self.y == other.y
    }
}

// PartialOrd 트레이트 구현: 두 Point를 <, >, <=, >= 연산자로 비교 가능하게 함
// 주로 x 값으로 비교하고, x가 같으면 y 값으로 비교
impl<T: PartialOrd, U: PartialOrd> PartialOrd for Point<T, U> {
    fn partial_cmp(&self, other: &Self) -> std::option::Option<std::cmp::Ordering> {
        // x 값을 먼저 비교
        match self.x.partial_cmp(&other.x) {
            Some(Ordering::Equal) => self.y.partial_cmp(&other.y),  // x가 같으면 y 비교
            other => other,  // x가 다르면 그 결과 반환
        }
    }
}