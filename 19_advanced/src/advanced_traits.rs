use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::Add;

// "연관 타입" 이란 트레잇 내부에 선언하여
// 트레잇 시그니처가 선언된 연관 타입을 사용할 수 있게 하는 것이다.
// 제너릭과 유사하지만 제너릭과 달리 여러 개의 구현을 막을 수 있다는 장점이 있다.
// 예를들어 제너릭을 사용할 경우 Iterator<String>, Iterator<Int> 등의 구현체가 있지만
// 연관 타입을 사용해서 구현체를 만들 경우, 하나의 구현체만 존재한다.

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    // Self 는 Trait 을 구현하는 타입을 의미한다.
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + (rhs.0 * 1000))
    }
}

pub fn example() {
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 }, Point {x: 3, y: 3});
}

// 트레잇 메소드명이 동일할 경우, 모호성을 없앨 수 있다.
struct Human;

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Pilot {
    fn fly(&self);
}

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

trait Wizard {
    fn fly(&self);
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!")
    }
}

// 모호성을 없애기 위해서 fully qualified 문법을 사용할 수 있다.
pub fn fully_qualified_example() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}

// Super trait 을 사용하여 다른 트레잇의 기능을 요구할 수 있다.
trait OutlinePrint: Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} * ", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct PointV2 {
    x: i32,
    y: i32,
}

impl Display for PointV2 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for PointV2 {

}

// 외부 타입에 대해 외부 트레잇을 구현하기 위한 뉴타입 패턴
// 어떤 트레잇을 구현하기 위해서는 트레잇 또는 대상 타입이 우리의 크레이트 내부에 있어야 하지만,
// 다음과 같이 Wrapper 를 이용한 뉴타입 패턴을 사용한다면 우회할 수 있다.
struct Wrapper(Vec<String>);

impl Display for Wrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

pub fn newtype_pattern_example() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w)
}