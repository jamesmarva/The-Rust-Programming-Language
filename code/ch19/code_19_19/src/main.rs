fn main() {
    let d  = Dog;

    // println!("A baby dog is called a {}", d.baby_name());
    println!("A baby dog is called a {}", Dog::baby_name());

    
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}