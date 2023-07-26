// 몇 가지 눈여겨 볼 만한 패턴 문법을 살펴본다.
pub fn example() {
    // 매치 가드 이용한 추가 조건
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("Less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no")
    }

    // Structure 해체
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 7 };
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y), // <-- 여기에 매칭된다.
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    // Enum 해체
    enum Message {
        Quit,
        ChangeColor(i32, i32, i32),
    }
    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        },
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }

    // @ 바인딩
    enum Msg {
        Hello { id: i32 }
    }
    let msg = Msg::Hello { id: 5 };
    match msg {
        Msg::Hello { id: id_variable @ 3..=7 } => {
            println!("Found an id in range: {}", id_variable)
        },
        Msg::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        },
        Msg::Hello { id } => {
            println!("Found some other id: {}", id)
        }
    }

    // 참조자 해체
    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 5 },
        Point { x: 10, y: -3 },
    ];
    let sum_of_squares: i32 = points
        .iter()
        .map(|&Point{ x, y }| x * x + y * y)
        .sum();
    println!("Sum of squares: {}", sum_of_squares);

    // 복잡한 구조 해체
    let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10});
    println!("Feet: {}, inches: {}, x: {}, y: {}", feet, inches, x, y);

    // .. 를 이용해서 나머지 부분 무시
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last)
        },
    }

    // "ref", "ref mut" 이용해서 패턴 내에서 참조자 생성
    // 패턴 매칭을 하는 경우 패턴 내 변수의 소유권이 이동하는데, 이를 막기 위해 "ref" 를 사용할 수 있다.
    let robot_name = Some(String::from("Bors"));
    match robot_name {
        Some(ref name) => println!("Found a name: {}", name),
        None => ()
    }
    let mut robot_name = Some(String::from("Bors"));
    match robot_name {
        Some(ref mut name) => *name = String::from("Another name"),
        None => ()
    }
    println!("robot_name is: {:?}", robot_name)
}