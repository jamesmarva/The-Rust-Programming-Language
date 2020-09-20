use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    let content = fs::read_to_string(config.filename)
                    .expect("omething went wrong reading the file");
    println!("{}", content);
    
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Config{ query, filename}
    }
}

