use std::fmt::{Display, Formatter, Result};

fn main() {
    let p = Point{
        x: 3, 
        y: 4,
    };
    println!("{}", p);
    println!("{}", p.to_string());
}

struct Point {
    x: u32,
    y: u32,
}

impl Display for Point {

    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}


