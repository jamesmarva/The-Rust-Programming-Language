use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    println!("Hello, world!");
    match read_username_from_file() {
        Ok(val) => {
            println!("val : {}", val);
        },
        Err(e) => panic!("{:?}", e)
    };
    match read_username_from_file_1() {
        Ok(val) => {
            print!("val : {}", val);
        },
        Err(e) => panic!("{:?}", e)
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_1() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    return Ok(s);
}
