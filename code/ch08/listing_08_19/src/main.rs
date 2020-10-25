fn main() {
    let ss = String::from("我勒个去");
    for c in ss.chars() {
        println!("{}", c);
    }
    let hello = "我勒个去"; 
    let s = &hello[0..3]; // s: type = &str
    println!("{}", s);

    let mut chars = ss.chars();
    // println!("{}", chars.next());
    // assert_eq!(chars.next(), Some('我'));
    // assert_eq!(chars.next().expect("hh"), '勒');
    let count = chars.count();
    // println!("{}", chars.next().expect("asdfasdf"));


}
