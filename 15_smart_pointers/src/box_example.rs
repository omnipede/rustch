pub fn box_example() {
    // Box 를 사용하면 힙에 데이터를 저장할 수 있다.
    // 즉, 스코프를 벗어나면 자동으로 해제된다.
    let b = Box::new(5);
    println!("b = {}", b);

    // Box 를 사용하면 재귀적 타입을 만들 수 있다.
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    // 1 -> 2 -> Nil
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("{:?}", list)
}

pub fn box_list_iteration() {
    struct Node {
        value: i32,
        next: Option<Box<Node>>
    }
    let node1 = Box::new(Node {
        value: 1,
        next: None,
    });
    let node2 = Box::new(Node {
        value: 2,
        next: Some(node1),
    });
    let node3 = Box::new(Node {
        value: 3,
        next: Some(node2),
    });

    let ptr = &node3;
    println!("{}", ptr.value);
    let next = &(ptr.next);
    let ptr = next.as_ref().unwrap();
    println!("{}", ptr.value);
    let next = &(ptr.next);
    let ptr = next.as_ref().unwrap();
    println!("{}", ptr.value);
}