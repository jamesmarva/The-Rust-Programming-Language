use std::fs::File;
use std::io::ErrorKind;

fn main() {
    println!("Hello, world!");

    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) =>file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                match File::create("hello2.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                }
            }
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };


    let f1 = File::open("hello.txt");
    let f1 = match f1 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            },
        },
    };

    // ---------------------------
    let f2 = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else (|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    }); 
}
