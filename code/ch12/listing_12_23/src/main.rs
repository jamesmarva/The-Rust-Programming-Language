use listing_12_23::run;
use listing_12_23::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let con = Config::new(&args).unwrap_or_else (|error| {
        println!("{}", error);
        process::exit(1);
    });

    run(con);
}
