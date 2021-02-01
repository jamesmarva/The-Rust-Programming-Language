trait Summary {
    fn summarize(&self) -> String;
}

impl <T> Summary for Vec<T> {
    fn summarize(&self) -> String {
        String::from("hello")
    }
}
fn main() {
    let vec: Vec<u32> = Vec::new();
    println!("{}", vec.summarize());
}
