fn main() {

    let weight = 30;
    let height = 50;
    
    println!("Hello, world!");
    println!("the area of rectangle: {}", get_area(weight, height));
    println!("{}, {}", weight, height);

}

fn get_area(weight : u32, height : u32) -> u32 {
    weight * height
}
