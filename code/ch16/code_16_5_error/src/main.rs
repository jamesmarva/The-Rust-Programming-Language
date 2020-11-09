fn main() {
    let v = vec![1, 4, 2, 4, 5];
    let handle = std::thread::spawn(move || {
        println!("vec: {:?}", v);   
    });
    drop(v);
    handle.join().unwrap();
}
