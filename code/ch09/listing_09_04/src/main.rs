use std::fs::File;


fn main() {
    

    let tmpFile  = File::open("hello1.txt");

    let file = match tmpFile {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };


}
