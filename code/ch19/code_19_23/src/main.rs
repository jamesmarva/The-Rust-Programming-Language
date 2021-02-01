use std::fmt::{Display, Formatter, Result};

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("james")]);
    println!("w = {}", w)

}

struct Wrapper(Vec<String>);

impl Display for Wrapper {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
