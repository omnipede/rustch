fn main() {
    // 참고로, 러스트는 제너릭에 대해 단형성화(monomorphization)를 수행하므로 런타임 비용이 없다.

    // 구조체 제너릭
    struct Point<T, U> {
        x: T,
        y: U,
    }
    let p = Point { x: 5, y: 4.0 };
    println!("{}, {}", p.x, p.y);

    // 메소드 제너릭
    impl <T, U> Point<T, U> {
        fn x(&self) -> &T {
            &self.x
        }

        fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }
    let p = Point { x: 5, y: 10.0 };
    println!("p.x = {}", p.x());
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
