use std::cell::RefCell;
use std::rc::{Rc, Weak};
use crate::cyclic_reference::List::{Cons, Nil};

// 러스트의 메모리 안정성은 메모리 릭을 대부분 방지하지만 완벽하지는 않다.
// 예를 들어 순환 참조가 발생하는 경우 각 아이템의 참조 카운트가 0이 되지 않아서 값이 버려지지 않게 된다.
pub fn cyclic_reference() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));
}

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match *self {
            Cons(_, ref item) => Some(item),
            Nil => None
        }
    }
}

// 순환 참조를 없애기 위해서 weak 스마트 포인터를 사용할 수 있다.
// weak count 가 0 이 아니더라도 scope 를 벗어나면 버려질 수 있다.
// weak pointer 가 참조하는 값이 버려졌을 수도 있기 때문에 upgrade 메소드를 호출해서 값이 존재하는지 확인해야 한다.
pub fn weak_smart_pointer() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![])
    });
    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)])
        });

        // leaf 의 부모로 branch 를 할당.
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("branch strong = {}, weak = {}", Rc::strong_count(&branch), Rc::weak_count(&branch));
        println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));

        // 순환 참조로 인해 무한히 출력되지 않는다.
        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}