use std::env;
use listing_12_23::Config;
use listing_12_23::run;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let con = Config::new(&args).unwrap_or_else (|error| {
        println!("{}", error);
        process::exit(1);
    });

    run(con);
}
