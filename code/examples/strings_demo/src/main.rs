fn main() {
    let s1 = String::from("hei hei ehei ");

    let s2: String = "hello  james".to_string();

    let s3: String = String::new();

    let s4 = String::from("asdfasdfasdfasdfa");

    let s5 = String::from(5.to_string());

    // the type str is unsize so complier doesn't check the code.
    // let str1: str = *"hello james";



}
