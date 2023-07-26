fn main() {
    let mut user1 = User {
        username: String::from("someone@example.com"),
        email: String::from("someusername123"),
        sign_in_count: 1,
        active: true,
    };
    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        // 구조체 갱신법
        ..user1
    };

    // 튜플 구조체
    let black = Color(0, 0, 0);

    // 어노테이션을 이용한 구조체 출력
    let rect = Rectangle {
        length: 50,
        width: 50
    };
    println!("rect is {:?}", rect);
    println!("rect is {:#?}", rect);
    println!("rect area is {}", rect.area());

    let another_rect = Rectangle {
        length: 50,
        width: 50
    };
    println!("rect can hold another_rect: {}", rect.can_hold(&another_rect));

    let square_rect = Rectangle::square(10);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

fn build_user(email: String, username: String) -> User {
    User {
        // 변수명과 필드명이 같을 때 간편하게 초기화 할 수 있다.
        email, username,
        active: true,
        sign_in_count: 1
    }
}

struct Color(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }

    // 연관 함수
    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }
}