fn main() {
    // let (x, y) = (1, 2, 3);
    let (x, y, _) = (1, 2, 3);
    println!("{}", x);
    println!("{}", y);
    
    let (x, y, ..) = (1, 2, 3);
    println!("{}", x);
    println!("{}", y);

}
