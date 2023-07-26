mod unsafe_super;
mod advanced_traits;
mod advanced_types;
mod advanced_func_closure;

fn main() {
    println!("---Unsafe---");
    unsafe_super::example();
    println!("---Traits---");
    advanced_traits::example();
    advanced_traits::fully_qualified_example();
    advanced_traits::newtype_pattern_example();
    println!("---Func---");
    advanced_func_closure::example();
}
