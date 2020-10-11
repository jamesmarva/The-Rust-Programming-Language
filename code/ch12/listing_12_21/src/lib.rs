use std::process;
use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String, 
    pub filename: String, 
}

impl Config {
    pub fn new (args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough argutments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config{
            query,
            filename,
        })
    }
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> vec<'a str> {
    
}


pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<'a str> {

}








