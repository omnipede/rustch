fn main() {
    // Vars (immutable, mutable)
    let x = 5;
    println!("The value of x is {}", x);
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);
    // const
    const MAX_POINTS: u32 = 100_000;

    // Types
    // ints: u8, u16, u32, u64, i8, i16, i32, i64
    // bool
    // char
    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    let a = tup.0;
    let b = tup.1;
    let c = tup.2;
    // array
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];

    // Functions
    let y = another_function(5);
    // inline expression
    let y = {
        let x = 3;
        x + 1
    };

    // Controls
    let x = 5;
    if x != 0 {
        println!("Number is not zero.")
    } else {
        println!("Number is zero.")
    }
    let condition = true;
    let x = if condition {
        5
    } else {
        6
    };
    // infinite loop
    // loop {
    //     println!("Here!")
    // }
    let mut x = 5;
    while x != 0 {
        println!("{}!", x);
        x = x - 1;
    }
    let a = [10, 20, 30];
    for elem in a.iter() {
        println!("Elem is {}", elem);
    }
    for n in (1..4).rev() {
        println!("{}!", n);
    }
}

fn another_function(x: i32) -> i32 {
    println!("Another function, x is {}", x);
    x + 1
}
