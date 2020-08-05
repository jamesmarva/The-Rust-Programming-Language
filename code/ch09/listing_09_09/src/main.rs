use std::fs;
use std::io;

fn main() {
    println!("Hello, world!");
}


fn read_usernaem_from_file() -> Result<String, io::Error> {
    let res = fs::read_to_string("hello.txt");
    return res;
}

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}