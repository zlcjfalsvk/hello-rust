
// static 변수는 const 와 다르게 메모리가 고정이며, mutable 할 수 있다
static mut COUNTER: u32 = 0;


fn main() {

    let mut num = 5;

    // 원시 포인터 선언
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // 같은 메모리 주소
    println!("r1 address is: {:?}", r1);
    println!("r2 address is: {:?}", r2);

    // 포인터가 가리키는 값에 접근하려고 할 때 유효하지 않은 값을 처리해야 할 수도 있는 경우 문제 발생
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // unsafe function 은 unsafe 블록 내에서만 호출 가능
    unsafe fn dangerous() {}
    unsafe {
        dangerous();
    }

    fn main() {
        let mut v = vec![1, 2, 3, 4, 5, 6];

        let r = &mut v[..];

        let (a, b) = r.split_at_mut(3);

        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);
    }

}

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}