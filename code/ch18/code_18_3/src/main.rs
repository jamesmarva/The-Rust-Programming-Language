fn main() {

    let v: Vec<char> = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{}, is at index {}", value, index);
    }
}
