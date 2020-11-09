use std::thread;
use std::sync::mpsc;

fn main() {

    let (transmitter, recevier) = mpsc::channel();
    let handle = thread::spawn(move || {
        let ss = String::from("sssssssssss");
        transmitter.send(ss).unwrap();
        println!("{}", ss);
    });

    let ss = recevier.recv().unwrap();
    println!("sdfasfdasdfafdas");

}
