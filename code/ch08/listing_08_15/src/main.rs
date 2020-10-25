fn main() {
    let mut s = String::from("foo");
    s.push_str("bar");

    let ss = "ddd";
    s.push_str(&ss);
    s.push_str(ss);
    println!("{}", s);
    println!("{}", s.to_string());
}
