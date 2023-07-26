
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

pub fn drop_example() {
    // 스코프를 벗어날 때 c 와 d 는 drop 된다.
    let c = CustomSmartPointer {
        data: String::from("my stuff!")
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff")
    };
    println!("CustomSmartPointers created.");
    // Double free 를 막기 위해 직접 c.drop() 을 호출하는 것은 허용되지 않는다.
    // 만약 명시적으로 drop 하길 원한다면 std::mem::drop 을 사용한다.
}