fn main() {
    let t = Test(1111);
    println!("t: {}", t.cat());
}


/// tuple struct
struct Test(usize);

impl Test {

    fn cat(&self) -> String {
        String::from(self.0.to_string())
    }
}
