use std::env;
use std::fs;

fn main() {

}

struct Config {
    query: String,
    filename: String,
}

fn run(config: Config) {
    let content = fs::read_to_string(config.filename)
        .expect("Something went gone when reading the file.");
    
}





