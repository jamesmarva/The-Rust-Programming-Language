use std::ops::Deref;

fn main() {
    println!("Hello, world!");
    let ss = "aaaa";
    let s = String::from("aaaa");
    
    println!("res: {}", s == ss);
    println!("res: {}", &s == ss);
    println!("res: {}", s == *ss);
    println!("res: {}", s == *ss);
    assert_eq!(ss, s);

    let arr1 = [String::from("fff"), String::from("ggg")];

    let vec1 = vec![String::from("fff"), String::from("ggg")];

    test_arr(&arr1);

    test_arr(&vec1);

    test_char_arr(&['a'; 4]);

    // &MyBox<String> => &String => &str
    hello(&MyBox::new(String::from("ffff")));
}

fn hello(name: &String) {
    println!("hell {}", name);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new (x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T>  Deref for MyBox<T> {

    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }

}

fn test_char_arr(arr: &[char]) {
    for item in arr.iter() {
        println!("{}", item);
    }
}

// 如果是借用就不用知道参数的表示的具体的大小，但是需要知道生命期限
fn test_arr(arr: &[String]) {

    for item in arr.iter() {
        println!("{}", item);
    }
}

// 如果是非借用的，就要知道具体的大小。
fn test_char_arr_backup(arr: [char; 10]) {
    for item in arr.iter() {
        println!("{}", item);
    }
}