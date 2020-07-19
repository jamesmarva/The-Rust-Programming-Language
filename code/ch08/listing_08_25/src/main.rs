fn main() {
    println!("Hello, world!");
    use std::collections::HashMap;


    let mut map = HashMap::new();
    map.insert(String::from("Blue"), 10);
    map.entry(String::from("Blue")).or_insert(50);
    map.entry(String::from("Yellow")).or_insert(50);
    println!("{:?}", &map);
    println!("{:?}", map);
}
