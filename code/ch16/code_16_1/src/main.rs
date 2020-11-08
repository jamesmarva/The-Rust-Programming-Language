use std::thread;
use std::time::Duration;

fn main() {

    thread::spawn(|| {
        // [1, 10)
        for i in 1..10 {
            println!("num {} in spawn thread.", i);
            thread::sleep(Duration::from_millis(1)); 
        }
    });

    // [1, 5)
    for i in 1..5 {
        let local_i = i;
        println!("local_i {}", local_i);
        thread::sleep(Duration::from_millis(5));
    }
}
