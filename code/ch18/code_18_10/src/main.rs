fn main() {
    if let x = 5 {
        println!("{}", x);
    }

    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("default"),
    }
}
