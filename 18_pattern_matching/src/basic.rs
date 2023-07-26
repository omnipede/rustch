pub fn example() {
    // match 값 {
    //  패턴 => 표현,
    //  패턴 => 표현
    // }
    let x = 17;
    match x {
        2 => {},
        3 => {},
        _ => {}
    }

    // if let
    let x = Some(5);
    if let Some(v) = x {
        println!("Value is {}", v)
    }

    // while let ***
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        println!("{}", top)
    }

    // for
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index{}", value, index);
    }

    // let PATTERN = EXPRESSION
    let (x, y) = (1, 2);

    // function parameters
    fn foo(x: i32) {

    }
    foo(12);
}