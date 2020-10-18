use std::env;
use std::process;
use listing_12_22::Config;
use listing_12_22::run;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|error| {
        println!("{}", error);
        process::exit(1);
    });
    run(config);
}
