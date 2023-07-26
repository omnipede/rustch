use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

// Channel 을 통해서도 동시성을 구현할 수 있지만, mutex 를 사용할 수도 있다.
// Channel 을 사용하면 데이터의 소유권이 옮겨지지만, mutex 를 사용하면 공유 데이터를 동시에 접근 가능하다.
// 하지만 channel 보다 복잡함.
pub fn mutex_example() {
    let m = Mutex::new(5);
    {
        // 데이터에 대해 락을 걸고 변경할 수 있다.
        // mutex 는 RefCell 과 유사하게 내부 가변성을 제공한다.
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);
}

pub fn multi_thread_mutex_example() {
    // Rc 는 멀티쓰레딩 환경에서 사용할 수 없으므로 Rc 대신 Arc (atomic reference count) 를 사용한다.
    // Rc 가 참조 카운트를 증가시키거나 감소시킬 때 동시성을 고려하지 않는다.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap())
}