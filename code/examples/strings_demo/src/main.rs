fn main() {
    let s1: String = String::from("hei hei ehei ");

    let s2: String = "hello  james".to_string();

    let s3: String = String::new();

    let s4 = String::from("asdfasdfasdfasdfa");

    let s5 = String::from(5.to_string());

    // the type str is unsize so complier doesn't check the code.
    // let str1: str = *"hello james";


    let s6: String = "tttt".to_string();
    let p = MyPointer {
        p: s6.as_str()
    };
    println!("{}", p.p);
    drop(s6);
    // println!("{}", p.p);
}


struct MyPointer<'a> {
    p: &'a str
}
