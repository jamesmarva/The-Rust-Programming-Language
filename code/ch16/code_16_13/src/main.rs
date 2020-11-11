use std::sync::Mutex;
use std::thread;

// error[E0382]: use of moved value: `counter`
// move occurs because `counter` has type `std::sync::Mutex<i32>`, which does not implement the `Copy` trait
// counter 这样的用法无法被多个线程的闭包使用。
fn main() {

    let counter = Mutex::new(0);

    let mut handles = vec![];

    // [0, 10)
    for _ in 0..10 {
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });

        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("result: {}", *counter.lock().unwrap())
}
