fn main() {
    println!("Hello, world!");

    let s = String::from("hahha i am the kind of my code");
    println!("{}", find_first_word(&s));
}

fn find_first_word(s : &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
