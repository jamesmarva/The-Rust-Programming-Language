fn main() {
 
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let res = longest(string1.as_str(), string2.as_str());
        println!("res: {}", res);
    }
}

fn longest<'a> (x: &'a str, y:&'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
