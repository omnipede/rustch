extern crate module;

pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {

            }
        }
    }
}

use a::series::of::nested_modules;

enum TrafficLight {
    Red, Yellow, Green
}

use TrafficLight::{Red, Yellow};

fn main() {
    module::client::connect();
    nested_modules();

    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;
}