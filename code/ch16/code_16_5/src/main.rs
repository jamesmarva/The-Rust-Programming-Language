fn main() {
    let v = vec![1, 3, 4, 5];
    let handle = std::thread::spawn(move || {
        println!("vec is {:?}", v);
    });
    handle.join().unwrap();
}
