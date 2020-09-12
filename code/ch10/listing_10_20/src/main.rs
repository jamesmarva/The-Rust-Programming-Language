fn main() {

    let s1 = String::from("abcd");
    let s2 = "xyz";

    let res = longest(s1.as_str(), s2);

    println!("the longer string: {}", res);
}
