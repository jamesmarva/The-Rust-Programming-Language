fn main() {
    unsafe {
        println!("abs", abs(-3));
    }
}

extern  "C" {
    fn abs(input: i32) -> i32;
}
