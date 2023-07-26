use std::fmt::Display;

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false
    };
    println!("1 new tweet: {}", tweet.summary());
    notify(tweet);

    let numbers = vec![34, 50, 25, 100, 65];

    let result = largest(&numbers);
    println!("The largest number is {}", result);

    let chars = vec!['y', 'm', 'a', 'q'];

    let result = largest(&chars);
    println!("The largest char is {}", result);
}

// 트레잇 바운드: 제너릭 타입을 특정한 트레잇으로 한정할 수 있다.
fn notify<T: Summarizable>(item: T) {
    println!("Breaking news1 {}", item.summary())
}

// 여러 개의 트레잇 바운드를 가질 수도 있다.
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// 트레잇 바운드를 이렇게 선언할 수도 있다.
fn smallest<T>(list: &[T]) -> T
    where T: PartialOrd + Copy {
    let mut smallest = list[0];
    for &item in list.iter() {
        if item < smallest {
            smallest = item;
        }
    }
    smallest
}

trait Summarizable {
    fn summary(&self) -> String;

    // Trait 메소드에 대해 기본 구현 가능
    fn summary_2(&self) -> String {
        String::from("(Read more...)")
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summarizable for NewsArticle {
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}