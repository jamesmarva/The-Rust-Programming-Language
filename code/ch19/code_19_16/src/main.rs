fn main() {
    let h = Human;
    h.fly();
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

// struct 
struct Human;

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}
impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}