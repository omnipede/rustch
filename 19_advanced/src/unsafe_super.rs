use std::slice;

// 가변 정적 변수 수정
static mut COUNTER: u32 = 0;

pub fn example() {

    // 가변 정적 변수 수정
    unsafe {
        COUNTER += 10;
        println!("COUNTER: {}", COUNTER);
    }

    // 로우 포인터 역참조하기
    let mut num = 5;
    let r1 = &num as *const i32; // 불변 포인터
    let r2 = &mut num as *mut i32; // 가변 포인터

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // 안전하지 않은 함수
    unsafe fn dangerous() {}
    unsafe {
        dangerous();
    }

    // 안전하지 않은 코드 상에 안전한 추상화 생성하기
    fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();
        assert!(mid <= len);
        // (&mut slice[..mid], &mut slice[mid..]) // <-- mutable borrow 를 두번 할 수 없다.
        // 따라서 raw pointer 를 이용해 slice 를 직접 나눈다.
        let ptr = slice.as_mut_ptr();
        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid)
            )
        }
    }

    // Extern 함수 사용하여 외부 코드 호출하기
    // 러스트는 FFI (Foreign Function Interface) 의 생성, 사용을 위해 'extern' 이라는 키워드가 존재한다.
    extern "C" {
        fn abs(input: i32) -> i32;
    }
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // 안전하지 않은 트레잇
    unsafe trait Foo {
        
    }
    unsafe impl Foo for i32 {

    }
}