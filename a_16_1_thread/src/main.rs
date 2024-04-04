use std::thread;
use std::time::Duration;

fn main() {
    let mut v = vec![];

    // 해당 thread 를 join().unwrap() 및 v 을 소유권(move keyword) 이전 없이 사용 가능
    thread::scope(|s| {
        let s = s.spawn(|| {
            for i in 1..=10 {
                // 1 <= i <= 10
                println!("hi number {} from the spawned thread!", i);

                v.push(i);

                println!("v: {:?}", v);

                thread::sleep(Duration::from_millis(1));
            }
        });
    });

    println!("now v: {:?}", v);

    for i in 1..5 {
        // 1 <= i < 5
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
