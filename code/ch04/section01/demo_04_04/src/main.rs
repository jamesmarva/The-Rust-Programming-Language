fn main() {
    println!("Hello, world!");
    let s1 = gives_ownership();

    let s2 = String::from("hello ");

    let s3 = takes_and_gives_back(s2);
}


fn gives_ownership() -> String {
    let some_thing = String :: from("hello hhhhhhhhhhh");
    some_thing
}

fn takes_and_gives_back(src_string : String) -> String {
    src_string
}
