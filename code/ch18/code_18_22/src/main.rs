fn main() {
    let s = Some(String::from("hello!"));

    if let Some(_) = s {
        println!("found a string");
    }
    println!("{:?}", s);
}
