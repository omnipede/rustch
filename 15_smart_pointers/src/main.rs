mod cyclic_reference;
mod box_example;
mod deref_example;
mod drop_example;
mod rc_example;
mod refcell_example;

use std::ops::Deref;
use crate::deref_example::{deref_coercion_example, deref_example};
use crate::box_example::{box_example, box_list_iteration};
use crate::cyclic_reference::{cyclic_reference, weak_smart_pointer};
use crate::drop_example::drop_example;
use crate::rc_example::{rc_count_example, rc_example};
use crate::refcell_example::refcell_example;

fn main() {
    println!("---Box---");
    box_example();
    box_list_iteration();
    println!("---Deref---");
    deref_example();
    deref_coercion_example();
    println!("---Drop---");
    drop_example();
    println!("---Rc---");
    rc_example();
    rc_count_example();
    println!("---RefCell---");
    refcell_example();
    cyclic_reference();
    weak_smart_pointer();
}