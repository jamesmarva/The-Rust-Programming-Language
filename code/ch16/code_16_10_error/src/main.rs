use std::thread;
use std::sync::mpsc;

fn main() {

    let vec = vec![2; 1111];
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        for i in vec {
            tx.send(i).unwrap();
            println!("{}, got into the channel", i);
        }
    });

    // thread::spawn(move || {
    //     let vec1 = vec![33, 333, 3333,33333];
    //     for i in vec1 {
    //         tx.send(i).unwrap();
    //     }
    // });
    handle.join().unwrap();
    for i in rx {
        println!("i : {}", i);
    }
}
