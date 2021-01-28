fn main() {


}

struct Counter {
    count:u32
}

impl Iterator for Counter {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.count)
    }
}


pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
