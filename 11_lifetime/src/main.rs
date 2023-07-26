fn main() {
    // lifetime 의 주 목적은 dangling pointer 를 방지하기 위함이다.
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // println!("r: {}", r);

    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago ...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
    println!("{}", i.part)

    // 라이프타임 생략 규칙
    // 1. 참조자인 파라미터는 각각 고유한 라이프타임 파라미터를 갖는다.
    // 2. 만약 하나의 라이프타임 파라미터만 있다면, 그 라이프타임이 모든 출력 라이프타임 파라미터에 대입된다.
    // 3. 만약 여러 개의 입력 라이프타임 파라미터가 있고, 그 중 하나가 &self, &mut self 라고 하면, self 의 라이프타임이
    //      모든 출력 라이프타임 파라미터에 대입된다.

    // 정적 라이프타임
    // 프로그램 전체의 라이프타임을 의미한다.
    let s: &'static str = "I have a static lifetime.";
}

// x 의 참조자를 반환할 지 y의 참조자를 반환할 지 알 수 없기 때문에
// lifetime 을 명시해야 한다.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 구조체 정의 상에서의 lifetime 명시
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// 메소드 정의 내에서의 lifetime 명시
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
