fn main() {
    let v1 = vec![1, 2, 3, 4];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("got: {}", val);
    }
}
