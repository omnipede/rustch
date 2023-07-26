use crate::messaging_example::{cloned_publisher_example, messaging_example, multiple_messaging_example};
use crate::mutex_example::{multi_thread_mutex_example, mutex_example};
use crate::thread_example::thread_example;

mod thread_example;
mod messaging_example;
mod mutex_example;
mod sync_and_send;

fn main() {
    // println!("---Basic---");
    // thread_example();
    // println!("---Channel---");
    // messaging_example();
    // multiple_messaging_example();
    // cloned_publisher_example();
    println!("---Mutex---");
    mutex_example();
    multi_thread_mutex_example();
}
