fn main() {
    unsafe {
        println!("abs", abs(-3));
    }
}

extern  "C" {
    fn abs(input: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
