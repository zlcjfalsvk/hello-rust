// const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
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
    let arr = [1,2,3,4,5]; // let arr: [i32; 5] = [1,2,3,4,5];
    let arr2 = [3; 5]; // let arr2 = [3,3,3,3,3];
    // compound type -----------------------
}

fn type_of<T>(_: &T) -> &'static str {
    return std::any::type_name::<T>();
}
