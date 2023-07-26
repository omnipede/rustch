pub fn example() {
    // 타입 별칭
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);

    // 동적인 크기의 타입 (Dynamic Sized Type, DST)
    fn generic<T: Sized>(t: T) {

    }
}