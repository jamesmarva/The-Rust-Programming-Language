use std::env;
use std::fs;

fn main() {
    let args: Vec<String>  = env::args().collect();

    let con = Config::new(&args);

    println!("{}",con.query);
    println!("{}", con.filename);

    let contents = fs::read_to_string(con.filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);


}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new (args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();
        Config{query, filename}
    }
}
