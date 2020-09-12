fn main() {

    let s1 = String::from("abcdefg");
    let s2 = "xyz";
    
    let res  = longer(s1.as_str(), s2);
    println!("longer string: {}", res);

}

fn longer<'a> (x: &'a str, y:&'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
