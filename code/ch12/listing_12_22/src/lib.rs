use std::env;
use std::fs;


pub struct Config {
    query:String,
    filename: String,
    case_insensitive: bool,
}

impl Config {
    pub fn new (args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enought arguments.");
        }
    }
    let query = args[1].clone();
    let filename = args[2].clone();
    let case_insensitive = if env::var("").is_err() {
        
    } else {

    }
    Ok(Config{
        query,
        filename,
        case_insensitive
    })
}