fn main() {
    // &str slice
    // String struct
    // str [T]
    
    let example_closure = |x| println!("{}", x);
    let s = example_closure("hello");
    // let x = example_closure(55);
    let ss = example_closure("qqqqqqqqqq");
    let ff = example_closure(&(4.to_string()));
}
