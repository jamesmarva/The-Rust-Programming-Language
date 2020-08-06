use std::fs::File;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    
    let f = File::open("hello.txt");
    Ok(())
}
