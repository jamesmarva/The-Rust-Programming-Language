fn main() {
    println!("Hello, world!");


    let mut s = String::from("hello world.");
    let last_index = find_the_last_index_of_first_word(&s);
    s.clear();
    //因为字符串被清除了，这样一来 last_index 就变的毫无意义了。

}

fn find_the_last_index_of_first_word(s : &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '  {
            return i;
        }
    }
    s.len()
}
