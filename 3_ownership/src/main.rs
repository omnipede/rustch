fn main() {
    // 소유권 규칙
    // 1. 각 값은 오너 변수를 갖는다
    // 2. 한번에 딱 하나의 오너만 존재할 수 있다.
    // 3. 오너가 스코프 밖으로 벗어날 때, 값은 버려진다.

    // 만약 s1, s2 가 동일한 힙 영역을 포인팅하면, 메모리 해제 시 두번 해제하게 되므로 에러가 발생한다.
    // 이를 막기 위해 다음 코드에서 s1 의 값의 소유권이 s2 로 이동 한다.
    let s1 = String::from("Hello");
    let s2 = s1;
    // println!("{}, world!", s1); // <-- Error!
    println!("{}, world!", s2);

    // 깊은 복사. 힙 영역을 그대로 복사한다.
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("{}, world!", s1);
    println!("{}, world!", s2);

    // 반면, 스택 영역 데이터는 그냥 복사가 된다.
    // i32 등은 Copy trait 를 가지므로 소유권이 이동하는 대신 복사된다.
    let x = 5;
    let y = x;
    println!("{} = {}", x, y);

    let s = String::from("Hello");
    takes_ownership(s);
    // println!("{}", s); // <-- Error!
    let x = 5;
    makes_copy(x);

    // 소유권이 필요하지 않은 경우, 참조자로 넘길 수 있다.
    // 이렇게 함수의 파라미터로 참조자를 넘기는 것을 빌림 (Borrow) 라고 한다.
    let s = String::from("Hello");
    calculate_length(&s);
    let mut s = String::from("Hello");
    change(&mut s); // 가변 참조자로 넘기면 수정 가능.

    // 가변 참조자는 스코프 내에 딱 하나만 만들 수 있다.
    // 또한 불변 참조자를 선언한 상태에서 가변 참조자를 선언할 수 없다.
    // 이는
    // Error!
    // let mut r = String::from("hello");
    // let r0 = &r;
    // let r1 = &mut r;
    // let r2 = &mut r;
    // println!("{}, {}, {}", r0, r1, r2);

    // Slice
    let mut s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    // s.clear(); <-- Error! 불변 참조자로 대여했기 때문에 가변 참조자 대여 .clear(&mut self) 는 할 수 없다.
    println!("{}, {}", hello, world)
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string)
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer)
}

fn calculate_length(s: &String) -> usize {
    // s.push_str(", world"); <-- Borrow 한 변수는 수정이 안된다.
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world"); // <-- mutable reference (가변 참조자) 로 선언하면 수정할 수 있다.
}
