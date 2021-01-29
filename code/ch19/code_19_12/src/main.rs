fn main() {
    let mut v: Vec<u32> = Vec::new();
    v.push(1212);
    v.push(12122);
    for i in v.iter() {
        println!("{}", i);
    }
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
