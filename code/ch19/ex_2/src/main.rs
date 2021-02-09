use std::ops::Range;
fn main() {
    let v: Range<usize>  = 0usize..20;
    let v1: Vec<Status> = v
        .map(Status::Value)
        .collect();
    for item in v1.iter() {
        match item {
            Status::Value(x) => println!("x:{}",x.to_string()),
            Status::Stop => {}
        }
    }
}

struct Value(usize);

impl Value {
    fn pri(&self) {
        println!("{}", self.0.to_string());
    }
}

enum Status {
    Value(usize),
    Stop,
}

impl Status {

}

