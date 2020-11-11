use std::sync::{Mutex, Arc};
use std::thread;

// Mutex 和 RefCell 同样具有内部可变性
fn main() {

    let counter = Arc::new(Mutex::new(0));

    let mut handles_vec = vec![];

    // [0, 10)
    for _ in 0..10 {
        let local_counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = local_counter.lock().unwrap();
            *num += 1;
        });
        handles_vec.push(handle);
    }

    for h in handles_vec {
        h.join().unwrap();
    }
    println!("result is {}", *counter.lock().unwrap());
}
