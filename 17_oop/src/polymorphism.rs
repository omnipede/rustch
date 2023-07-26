// Rust 는 상속 대신 트레잇을 이용해서 다형성을 구현한다.
// 트레잇 객체란 특정 트레잇을 구현한 타입의 인스턴스를 의미한다.
// 그리고 객체 안전한 트레잇만 트레잇 객체로 만들 수 있다.
// 객체 안전한 트레잇이란,
//  반환값이 'Self' 가 아니고
//  제너릭 타입 매개변수가 없어야 한다.
pub fn example() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ]
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            })
        ]
    };
    screen.run();
}


trait Draw {
    fn draw(&self);
}

struct Screen {
    components: Vec<Box<dyn Draw>>
}

impl Screen {
    fn run(&self) {
        for component in self.components.iter() {
            component.draw()
        }
    }
}

struct Button {
    width: u32,
    height: u32,
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Draw button!");
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Draw select box!");
    }
}