#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    println!("Hello, world!");

    let rec = Rectangle { 
        width: 30,
        height: 50,
    };
    println!("struct: {:?}, the result is {}", rec, get_area_of_rectangle(&rec));
}

fn get_area_of_rectangle(rectangle : & Rectangle) -> u32 {
    rectangle.width * rectangle.height
}




