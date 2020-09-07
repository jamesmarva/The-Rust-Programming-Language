fn main() {
   let s1 = String::from("hhhhhhhhhhhhhhh");
    let (s2, len) = calculate_length(s1);
    println!("length {}, s: {}", len, s2);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
