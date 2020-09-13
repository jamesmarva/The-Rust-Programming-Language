fn main() {
    let s1 = "long string is long";
    let res;
    {
        let s2 = String::from("xyz");
        res = longest(s1, s2.as_str());
    }
    println!("res: {}", res);
}

fn longest<'a>(a: &'a str, b:&'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
