use std::ops::Add;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        for i in 1..10 {
            let val = String::from("hi").add(" ").add(&*i.to_string());
            println!("{val}");

            tx.send(val).unwrap();
        }
    })
    .join()
    .unwrap();

    // let received = rx.recv().unwrap();
    // println!("Got: {}", received); // hi 1

    rx.iter().for_each(|r| {
        println!("Got: {}", r);
    });

    let (tx, rx) = mpsc::channel();

    // 송신자 복사
    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
