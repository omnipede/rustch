use std::{i32, thread};
use std::time::Duration;

fn main() {
    // Closure 란 환경을 캡처할 수 있는 익명 함수를 의미한다.
    let add_one = |x| -> i32 {
        x + 1
    };
    let r = add_one(1);
    println!("{}", r);

    // 값을 캐싱하는 객체
    let mut cacher = Cacher::new(|num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    // 값이 캐싱되어 있으므로 한번만 계산한다.
    let r = cacher.value(32);
    let r = cacher.value(32);
    println!("{}", r);

    // 클로저로 환경 캡쳐하기
    let x = 4;
    let equal_to_x = |z| z == x;
    // fn equal_to_x_2(z: i32) -> bool { z == x } // <-- 일반 함수는 환경 캡쳐가 안된다. 오직
    let y = 4;
    assert!(equal_to_x(y));

    // 반복자 (iterator) 예시
    // 반복자는 "Iterator" 트레이트를 구현한다.
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    // 클로저를 이용한 반복자 필터링
    let v = vec![10, 20, 30];
    let filtered: Vec<i32> = v.into_iter().filter(|s| s >= &20).collect();

    // Custom iterator
    struct Counter {
        count: u32,
    }

    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;
            if self.count < 6 {
                Some(self.count)
            } else {
                None
            }
        }
    }

    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

// 클로저를 제너릭 변수로 갖는 구조체
struct Cacher<T> where T: Fn(u32) -> u32 {
    calculation: T,
    value: Option<u32>,
}

impl <T> Cacher<T> where T:Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation, value: None
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
