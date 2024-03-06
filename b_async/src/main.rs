use std::time::Duration;
use std::thread;
use async_std::task; // async-std 라이브러리 사용

// 간단한 비동기 Future 구현
#[derive(Debug)]
struct SimpleFuture {
    completed: bool,
}

impl SimpleFuture {
    async fn new() -> Self {
        // 비동기적인 작업 시뮬레이션을 위해 sleep 사용
        task::sleep(Duration::from_secs(2)).await;
        SimpleFuture { completed: true }
    }
}

// 비동기 함수
async fn async_function() {
    println!("Start of async function");

    // 비동기 Future를 호출
    let result = SimpleFuture::new().await;

    println!("Result of async function: {:?}", result);
}

fn main() {
    // 비동기 함수 호출
    let async_handle = task::block_on(async_function());

    // 다른 작업 수행 가능
    println!("Main function doing some work");

    // 비동기 함수의 결과 출력
    println!("Main function after waiting: {:?}", async_handle);
}