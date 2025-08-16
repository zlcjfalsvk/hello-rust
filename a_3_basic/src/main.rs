// Rust 기본 문법 학습
// 변수, 상수, 섀도잉, 복합 타입, 조건문과 반복문

// 상수 선언 예제 - 컴파일 타임에 값이 결정되며, 전역 스코프에서 사용 가능
// const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // 함수 호출 예제 - 레이블이 있는 측정값 출력
    print_labeled_measurement(5, 'h');

    // 가변 변수(mutable variable) 선언과 사용
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;  // mut 키워드로 인해 값 변경 가능
    println!("The value of x is: {x}");

    // ===== Shadowing (섀도잉) =====
    // 섀도잉: 같은 이름의 변수를 다시 선언하여 이전 변수를 가리는 기법
    // mut과 달리 다른 타입으로도 변환 가능하며, 불변성을 유지하면서 값 변경 가능
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {}", x);
    let x = x.to_string() + " ";
    println!("x typeof: {}", type_of(&x));
    // shadowing -----------------------

    // ===== 복합 타입 (Compound Types) =====
    // 튜플(Tuple): 여러 타입의 값들을 하나로 묶은 복합 타입
    let tup = (500, 6.4, 1);
    // 튜플 구조 분해(destructuring)를 통한 값 접근
    let (x, y, z) = tup;
    println!("The value of y is: {y}");
    // 인덱스를 통한 직접 접근 (0부터 시작)
    println!("The value of z is: {}", tup.2);

    // 배열(Array): 같은 타입의 고정된 크기를 가진 요소들의 모음
    // 배열은 스택에 할당되며, 컴파일 시점에 크기가 결정됨
    let arr = [1, 2, 3, 4, 5]; // 타입 명시: let arr: [i32; 5] = [1,2,3,4,5];
    let arr2 = [3; 5]; // 같은 값으로 초기화: [3,3,3,3,3]
    // 주의: 배열 범위를 벗어난 인덱스 접근 시 런타임 패닉 발생

    // ===== 조건문과 반복문 =====
    // if-else 조건문: 조건은 반드시 bool 타입이어야 함
    if arr.is_empty() {
        println!("arr length: {}", arr.len());
    } else if arr.len() - arr2.len() == 0 {
        println!("arr.len - arr2.len = {}", arr.len() - arr2.len());
    }

    // 표현식으로서의 if: 변수에 조건부 값 할당 가능
    let condition = true;
    let number = if condition { 5 } else { 6 };  // 두 분기의 타입이 같아야 함
    println!("The value of number is: {number}");

    // Rust의 반복문 종류: loop(무한), while(조건), for(반복자)
    // Loop 레이블: 중첩된 루프에서 특정 루프를 break/continue 할 때 사용
    let mut counter = 0;
    // 'first는 루프 레이블, break 시 값 반환 가능
    let result = 'first: loop {
        counter += 1;
        if counter == 10 {
            break 'first counter * 2;  // 레이블과 함께 값 반환
        }
    };
    println!("The result is {result}");

    let a = [10, 20, 30, 40, 50];

    // for 루프와 enumerate(): 인덱스와 값을 동시에 반복
    // iter()는 배열의 반복자를 생성, enumerate()는 (인덱스, 값) 튜플 생성
    for (i, element) in a.iter().enumerate() {
        println!("index: {i}, value: {element}");
    }

    // Range와 rev(): 1..4는 1,2,3을 생성 (4는 제외)
    // rev()는 순서를 뒤집어 3,2,1 순으로 반복
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

// 제네릭 함수: 타입 정보를 문자열로 반환
// <T>는 제네릭 타입 파라미터, &T는 T 타입의 참조
// 'static은 반환된 문자열이 프로그램 전체 생명주기 동안 유효함을 의미
fn type_of<T>(_: &T) -> &'static str {
    // Rust에서 함수의 마지막 표현식은 세미콜론 없이 작성하면 반환값이 됨
    // return std::any::type_name::<T>(); 와 동일
    std::any::type_name::<T>()
}

// 파라미터가 있는 함수 예제
// 모든 파라미터는 타입을 명시해야 함
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
