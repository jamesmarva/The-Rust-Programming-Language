fn main() {
    
    let p = Point{
        x: 0, 
        y: 7,
    };

    let Point{x: a, y: b} = p;
    println!("{:?}", p);
    assert_eq!(0, a);
    assert_eq!(7, b);
}

#[derive(Debug)]
struct Point {
    x: i32, 
    y: i32, 
}
