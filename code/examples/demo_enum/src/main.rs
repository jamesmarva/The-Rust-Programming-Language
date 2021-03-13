fn main() {
    enum Animal {
        dog = 1,
        cat = 200,
        chicken,
        tiger,
    }
    println!("tiger is: {}", Animal::tiger as isize);
    println!("dog is: {}", Animal::dog as isize);
    println!("cat is: {}", Animal::cat as isize);
}
