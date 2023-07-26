use std::ops::Deref;

// 스마트 포인터는 "Deref" 와 "Drop" 트레이트를 구현한다.
// "Deref" 트레이트는 역참조 연산 (*) 을 가능하게 한다.
pub fn deref_example() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    // rust 뒤쪽에서는 다음과 같은 코드가 실행된다.
    // *y = *(y.deref())
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// 역참조 강제
// Deref 를 구현한 타입의 참조자를 Deref 를 구현한 타입이 포함하고 있는 타입의 참조자로 바꾸어주는 기능
pub fn deref_coercion_example() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    hello(&(*m)[..]); // <-- 역참조 강제 덕분에 굳이 이렇게 호출하지 않아도 된다.
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}