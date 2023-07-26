use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// 메시지 패싱이란 스레드 간에 메시지를 주고 받는 것을 의미한다.
pub fn messaging_example() {
    // 복수 생성자, 단수 소비자 (multiple producer, single consumer)
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // channel 을 통해 전송하면 소유권이 바뀐다.
        // println!("val is {}", val); // <-- 따라서 사용 불가
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

pub fn multiple_messaging_example() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received)
    }
}

pub fn cloned_publisher_example() {
    let (tx, rx) = mpsc::channel();
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
        println!("Got: {}", received)
    }
}