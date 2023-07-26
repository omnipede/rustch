// 반증 불가 (irrefutable): 주어진 어떠한 값에도 대응되는 패턴. let x = 5;
// 반증 가능 (refutable): 주어진 값에 대응이 실패할 수 있는 패턴. if let Some(x) = 5
// let, for, 함수 매개변수는 반증 불가능한 패턴만 허용
// if let, while let 은 반증 가능한 패턴만 허용한다.
pub fn example() {

    // ex. 여기에는 반증 불가능한 패턴만 사용해야 한다.
    // let Some(x) = 5

    // ex. 여기에는 반증 가능한 패턴만 사용해야 한다.
    // if let x = 5 {
    //     println!("{}", x);
    // };
}