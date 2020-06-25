use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let answer = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("please input your guessing number: ");
        let mut guessing_str = String::new();
        io::stdin()
            .read_line(&mut guessing_str)
            .expect("Failed to read line.");
        let guess_num : u32 = match guessing_str.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess_num.cmp(&answer) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too greater!"),
            Ordering::Equal => {
                println!("you get it!");
                break;
            }
        }
        println!("----------------------------------")
    }
}
