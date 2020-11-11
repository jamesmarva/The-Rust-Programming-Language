use std::sync::Mutex;
use std::thread;
use std::rc::Rc;

// error[E0277]: `std::rc::Rc<std::sync::Mutex<i32>>` cannot be sent between threads safely
// help: within `[closure@src\main.rs:13:36: 16:10 local_counter:std::rc::Rc<std::sync::Mutex<i32>>]`, 
// the trait `std::marker::Send` is not implemented for `std::rc::Rc<std::sync::Mutex<i32>>`
fn main() {
    let counter = Rc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let local_counter = Rc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = local_counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("result: {}", *counter.lock().unwrap());
}
