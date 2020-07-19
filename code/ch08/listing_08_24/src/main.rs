fn main() {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    map.insert(&"Blue", 10);
    map.insert(&"Blue", 25);
    println!("{:?}", map.get(&"Blue").unwrap()); 
    println!("{:?}", map);

}
