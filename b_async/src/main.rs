// 비동기 프로그래밍 학습
// async/await와 Future 개념 이해

use async_std::task;  // async-std: 비동기 런타임 라이브러리
use std::thread;
use std::time::Duration;

// Future 타입을 구현하는 간단한 구조체
// Future는 아직 완료되지 않은 비동기 작업을 나타냄
#[derive(Debug)]
struct SimpleFuture {
    completed: bool,  // 작업 완료 여부를 나타내는 필드
}

impl SimpleFuture {
    // async 키워드: 이 함수가 Future를 반환함을 의미
    async fn new() -> Self {
        // 비동기 sleep: 2초 동안 대기 (블로킹하지 않음)
        // await: Future가 완료될 때까지 대기
        task::sleep(Duration::from_secs(2)).await;
        
        // sleep이 완료된 후 SimpleFuture 반환
        SimpleFuture { completed: true }
    }
}

// 비동기 함수 예제
async fn async_function() {
    println!("Start of async function");

    // SimpleFuture::new()는 Future를 반환
    // await를 사용하여 Future가 완료될 때까지 대기
    let result = SimpleFuture::new().await;

    println!("Result of async function: {:?}", result);
}

fn main() {
    // main 함수는 동기 함수이므로 async 함수를 직접 호출할 수 없음
    // block_on: 비동기 코드를 동기적으로 실행 (블로킹)
    let async_handle = task::block_on(async_function());

    // block_on이 완료된 후 실행됨 (비동기 함수가 모두 완료된 후)
    println!("Main function doing some work");

    // async_handle은 () 타입 (async_function의 반환값)
    println!("Main function after waiting: {:?}", async_handle);
    
    // 주의: 현재 코드는 block_on으로 인해 실제로는 동기적으로 실행됨
    // 진정한 비동기 동시성을 위해서는 task::spawn 등을 사용해야 함
}
