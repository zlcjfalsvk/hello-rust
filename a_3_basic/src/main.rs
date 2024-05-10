// const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    print_labeled_measurement(5, 'h');

    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // shadowing -----------------------
    // mut 과 다르게 다른 타입으로 치환할 수 있음
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

    // compound type(tuple, array) -----------------------
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");
    println!("The value of z is: {}", tup.2);

    // arr 일 경우 arr[0] 식으로 접근 가능
    // 배열 크기 이상의 주소를 가져와 에러가 발생할 경우 Panic 처리를 해야 함
    let arr = [1, 2, 3, 4, 5]; // let arr: [i32; 5] = [1,2,3,4,5];
    let arr2 = [3; 5]; // let arr2 = [3,3,3,3,3];
                       // compound type -----------------------

    // conditional & loop -----------------------
    if arr.is_empty() {
        println!("arr length: {}", arr.len());
    } else if arr.len() - arr2.len() == 0 {
        println!("arr.len - arr2.len = {}", arr.len() - arr2.len());
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    // 반복문에는 loop(무한 루프), while, for 가 있음
    // loop label
    let mut counter = 0;
    let result = 'first: loop {
        counter += 1;
        if counter == 10 {
            break 'first counter * 2;
        }
    };
    println!("The result is {result}");

    let a = [10, 20, 30, 40, 50];

    for (i, element) in a.iter().enumerate() {
        println!("index: {i}, value: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
    // conditional -----------------------
}

fn type_of<T>(_: &T) -> &'static str {
    // 함수의 반환은 return 및 `;` 없이 선언 가능
    // return std::any::type_name::<T>();
    std::any::type_name::<T>()
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
