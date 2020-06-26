// Functions with ownership and scope annotated
fn main() {
    println!("Hello, world!");
    let s = String::from("hello");
    takes_ownership(s);             // s's value moves into the function...
    // println!("{}", s);
    let x = 5;

    makes_copy(x);

}

fn takes_ownership(some_string : String) {
    println!("some_string is: {}", some_string);
}

fn makes_copy(some_integer : i32) {
    println!("some_integer is: {}", some_integer);
}