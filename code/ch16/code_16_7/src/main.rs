use std::thread;
use std::sync::mpsc;

fn main() {

    let (transmitter, receiver) = mpsc::channel();

    let handle = thread::spawn(move || {
        let val = String::from("asdfsafdasdfasdfasf");
        transmitter.send(val).unwrap();
    });
}
