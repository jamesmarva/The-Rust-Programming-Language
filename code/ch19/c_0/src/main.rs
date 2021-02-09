fn main() {

}


fn returns_clousure() -> dyn Fn(i32) -> i32 {
    |x| x + 1
}