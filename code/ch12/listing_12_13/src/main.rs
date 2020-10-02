use std::env;
use std::process;

use listing_12_13::Config;
fn main() {


    let args: Vec<String>  = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|error| {
        println!("Problem parsing arguments: {}", error);
        process::exit(1);
    });

    if let Err(e) = listing_12_13::run(config) {
        println!("application error: {}", e);
        process::exit(1);
    }
}
