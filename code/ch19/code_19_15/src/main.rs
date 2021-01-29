
use std::ops::Add;
#[derive(Debug)]
struct Millimeters(u32);

#[derive(Debug)]
struct Meters(u32);

impl Add for Millimeters {
    type Output = Millimeters;
    fn add(self, o: Millimeters) -> Millimeters {
        Millimeters(self.0 + o.0)
    }
}

impl Add for Meters {
    type Output = Meters;
    fn add(self, o: Meters) -> Meters {
        Meters(self.0 + o.0)
    }
}

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, o: Meters) -> Millimeters {
        Millimeters(self.0 + (o.0 * 1000))
    }
}

impl Add<Millimeters> for Meters {
    type Output = Millimeters;

    fn add(self, o: Millimeters) -> Millimeters {
        Millimeters(self.0 * 1000 + o.0)
    }
}


fn main() {
    let a = Meters(1);
    let b = Millimeters(1111);
    let c = b + a;
    println!("{:?} ", c);
    let d = Meters(3) + Millimeters(2222);
    println!("{:?}", d);
    let e = Meters(3) + Meters(2222);
    println!("{:?}", e);
    let f = Millimeters(3) + Millimeters(2222);
    println!("{:?}", f);

}
