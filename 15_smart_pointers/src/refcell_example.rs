use std::cell::RefCell;
use std::rc::Rc;
use crate::refcell_example::List::{Cons, Nil};

// 내부 가변성이란 어떤 데이터에 대해 불변 참조자가 있더라도 데이터를 변형할 수 있게 해주는 러스트 패턴이다.
// 참조 규칙에 따르면 어떤 데이터에 대해 불변 참조자를 여러 개 가지거나 가변 참조자를 하나 가질 수 있다. 그리고 두 가지 규칙은 동시에 적용될 수 없다.
// 이러한 규칙을 깰 필요가 있을 때 RefCell 을 사용한다.
pub fn refcell_example() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    *((*value).borrow_mut()) += 10;
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

