use std::thread;
use std::time::Duration;

pub fn thread_example() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    // 모든 작업이 끝날 때 까지 대기한다.
    handle.join().unwrap();

    for i in 1..5 {
        println!("Hi: {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    // 외부 변수를 사용하려면 move 를 사용해야 한다.
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();
}