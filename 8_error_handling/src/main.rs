use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};
use std::net::IpAddr;

fn main() {
    // panic!("crash and burn");

    // let v = vec![1, 2, 3];
    // v[99]; // <-- Panic!

    // 기본 에러 헨들링
    // let f = File::open("hello.txt");
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("There was a problem opening the file: {:?}", error)
    //     }
    // };

    // 서로 다른 에러에 대해 매칭하기
    // let f = File::open("hello.txt");
    // let f = match f {
    //     Ok(file) => file,
    //     Err(ref error) if error.kind() == ErrorKind::NotFound => {
    //         match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => {
    //                 panic!("Tried to create file but there was a problem: {:?}", e)
    //             }
    //         }
    //     },
    //     Err(error) => {
    //         panic!("There was a problem opening the file: {:?}", error)
    //     }
    // };

    // 에러 발생 시 패닉을 위한 숏컷 unwrap, expect
    // let f = File::open("hello.txt").unwrap();
    let f = File::open("hello.txt").expect("Failed to open hello.txt"); // <-- 에러 메시지 선택 가능

    // 언제 패닉 처리를 해야 하는가?
    // 예제, 프로토 타입 코드, 테스트는 패닉을 일으켜도 괜찮다
    // 컴파일러보다 개발자가 더 많은 정보를 가지고 있을 때. 예를 들어 다음과 같은 경우 패닉 처리를 해도 괜찮다.
    let home = "127.0.0.1".parse::<IpAddr>().unwrap();
}

// 에러 전파 방법
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }
}

// 숏컷 "?" 을 이용한 에러 전파 방법. 단 "?" 는 Result 를 반환하는 함수에서만 사용 가능하다.
fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// 숏컷 "?" 의 체이닝 기능을 이용한 에러 전파 방법
fn read_username_from_file_3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}