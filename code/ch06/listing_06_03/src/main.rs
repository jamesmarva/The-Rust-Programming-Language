enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}


fn value_in_cents(coin : Coin) -> u8 {
    // println!("{}", coin);
    match coin { 
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
fn main() {
    println!("Hello, world!");
    let res : u8 = value_in_cents(Coin :: Quarter);
    println!("res : {}", res);
}
