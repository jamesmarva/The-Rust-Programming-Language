fn main() {

    let p = Point{x: 7, y: 7};

    // match p {
    //     Point {x, y: 0} => println!("On the x axis at {}", x), 
    //     Point {x: 0, y} => println!("On the y axis at {}", y),
    //     Point {x, y} =>  println!("On neither axis: ({}, {})", x, y), 
    // }

    match p {
        Point {x, y: 0} => println!("On the x axis at {}", x), 
        Point {x: 0, y} => println!("On the y axis at {}", y),
        _ =>  println!(""), 
    }
}

struct Point {
    x: i32, 
    y: i32,
}
