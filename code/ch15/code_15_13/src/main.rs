fn main() {
    let m = MyBox::new(String::from("rust"));
    hello(&(*(m.myDeref()))[..]);
    hello(&(*m.myDeref())[..])
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }

    fn myDeref(&self) -> &T {
        &self.0
    }
}


fn hello(s: &str) {
    println!("{}", s);
}
