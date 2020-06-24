
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    println!("Hello, world!");
    let rec1 = Rectangle {
        width: 30, 
        height: 50,
    };

    println!("the area of rectangle: {}", get_area_of_rectangle(&rec1));
}

fn get_area_of_rectangle(rectangle: &Rectangle) -> u32 {
    return rectangle.width * rectangle.height;
}
