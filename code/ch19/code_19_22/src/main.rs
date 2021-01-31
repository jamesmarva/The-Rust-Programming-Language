use std::fmt::{Display, Formatter, Result};

trait OnlinePrint: Display {
    fn online_print(&self) {
        let str = self.to_string();
        let len = str.len();
        println!("{}", "*".repeat(len + 4)); 
        println!("*{}*", " ".repeat(len + 2)); 
        println!("* {} *", str);
        println!("*{}*", " ".repeat(len + 2)); 
        println!("{}", "*".repeat(len + 4)); 
    }
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


struct Position {
    longitude: f32,
    latitude: f32,
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {})", self.longitude, self.latitude)
    }
}

impl OnlinePrint for Point {}

fn main() {
    let p = Point{
        x: 1111,
        y:222,
    };
    
    p.online_print()
}
