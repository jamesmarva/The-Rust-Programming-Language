fn main() {

    let h = Human;
    h.fly();
    Pilot::fly(&h);
    Wizard::fly(&h);
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Human {
    fn fly(&self) {
        println!("human fly");
    }
}

impl Pilot for Human {
    fn fly(&self) {
        println!("Pilot fly")
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Wizard fly")
    }
}
