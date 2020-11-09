use std::thread;


fn main() {

    let v = vec![1, 3, 4];

    let handle = thread::spawn(||{
        println!("vector : {:?}", v);
    });
}
