fn main() {
    println!("Hello, world!");
    let v = vec![1, 2, 3, 4, 5, 6];

    let third: &i32 = &v[2];

    println!("the third element is {}", third);

    match v.get(1) {
        Some(first) => println!("the first ele is {}", first),
        None => println!("there is no fisst ele"),
    }
}
