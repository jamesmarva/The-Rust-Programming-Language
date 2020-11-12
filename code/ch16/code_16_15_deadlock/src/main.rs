use std::sync::{Mutex, Arc};
use std::thread;
use std::time::Duration;

fn main() {
    let first = Arc::new(Mutex::new("lock1"));
    let first_clone = Arc::clone(&first);

    let second = Arc::new(Mutex::new("lock1"));
    let second_clone = Arc::clone(&second);

    let handle1 = thread::spawn(move || {
        let first_result = first.lock().unwrap();
        thread::sleep(Duration::from_secs(3));
        let second_result = second.lock().unwrap();

        let res = first_result.to_string() + *second_result;
        println!("first res: {}", res);
    });

    let handle2 = thread::spawn(move || {
        let second_result = second_clone.lock().unwrap();
        thread::sleep(Duration::from_secs(3));
        let first_result = first_clone.lock().unwrap();

        let res = first_result.to_string() + *second_result;
        println!("second res: {}", res);
    });
    handle1.join().unwrap();
    handle2.join().unwrap();
}
