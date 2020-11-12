use std::sync::{Mutex, Arc};
use std::thread;
use std::time::Duration;

fn main() {
    let first_lck = Arc::new(Mutex::new("first lock"));
    let second_lck = Arc::new(Mutex::new("second_lock"));

    let first_lck_clone = Arc::clone(&first_lck);
    let second_lck_clone = Arc::clone(&second_lck);

    let handle1 = thread::spawn(move || {
        let first = first_lck.lock().unwrap();
        thread::sleep(Duration::from_secs(2));
        let second = second_lck.lock().unwrap();
        let res = (*first).to_string() + *second;
        println!("handle1 : {}", res);
    });

    let handle2 = thread::spawn(move || {
        let 
    })
    
}
