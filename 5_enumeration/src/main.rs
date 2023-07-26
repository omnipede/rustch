fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("Hello world"));
    m.call();

    // 패턴 매칭
    let coin = Coin::Quarter(UsState::Alabama);
    let v = value_in_cents(coin);

    // 매칭 시 모든 패턴을 처리해야 한다.
    let v = 8;
    match v {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => () // Placeholder _ 를 사용해서 나머지 값을 매칭한다
    }

    // match 문법은 모든 패턴을 고려하는 불편함이 있으므로 이를 해결하기 위해 if let 구문을 사용할 수 있다.
    let v = Some(8);
    if let Some(3) = v {
        println!("three");
    }
}

enum IpAddrKind {
    V4, V6
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {

    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ...etc
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // 값을 바인딩하는 패턴
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}