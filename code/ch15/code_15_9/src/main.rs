fn main() {
    let s = "aaa";
    println!("{}", &s[0..1]);
}

struct MyBox<T> (T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}