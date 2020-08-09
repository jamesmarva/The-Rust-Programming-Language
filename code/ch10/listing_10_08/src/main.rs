struct Point<T, U> {
    x: T,
    y: U,
}
fn main() {
    println!("Hello, world!");
    let point = Point {
        x: 4,
        y: 5.0
    };

    let p2 = Point{
        x: 4.0,
        y:6
    };
    println!("{}", point.x);
    println!("{}", p2.y);
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };

}
