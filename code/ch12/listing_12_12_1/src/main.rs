use std::env;
use std::fs;
use std::process;
use std::error::Error;


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|error| {
        println!("Problem parsing arguments: {}", error);
        process::exit(1);
    }); 
    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new (args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments.");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config{
            query,
            filename,
        })
    }       
}
fn run (con: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(con.filename)?;
    println!("contents: \n{}", contents);
    Ok(())
}



