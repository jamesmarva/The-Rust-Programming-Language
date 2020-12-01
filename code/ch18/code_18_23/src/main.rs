fn main() {
    struct Point {
        x: i32, 
        y: i32, 
        z: i32, 
    }
    let x_val = Point{x: 0, y: 0, z: 0};

    match x_val {
        Point{x, ..} => println!("x is {}", x),
    }

    let y_val = x_val;
    match y_val {
        Point{x:_, y, z:_} => println!("y is {}", y),
    }

    let z_val = y_val;
    match z_val {
        Point{x:_, y:_, z} => println!("z is {}", z),
    }
}
