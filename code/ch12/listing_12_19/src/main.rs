use listing_12_19::run;
use std::env;
use listing_12_19::Config;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let con = Config::new(&args).unwrap_or_else (|error| {
        println!("{}", error);
        process::exit(1);
    });
    run(con);
}
