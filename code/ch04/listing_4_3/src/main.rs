fn main() {
    let s = String::from("hellO");

    takes_ownership(s);
    // println!("s ==== {}", s); happen error
    make_copy(111111);

}

fn takes_ownership(s: String) {
    println!("s = {}", s);
}

fn make_copy(i: i32) {
    println!("i = {}", i);
}
