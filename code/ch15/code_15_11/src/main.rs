use std::ops::Deref;

fn main() {
    
}

fn hello(name: &str) {
    println!("hello, {}", name);
}

struct MyBox<T> (T);


impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}