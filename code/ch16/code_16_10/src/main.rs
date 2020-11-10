use std::thread;
use std::sync::mpsc;
use std::time::Duration;

//发送多个值并观察接收者的等待
fn main() {
    let (transmitter, receiver) = mpsc::channel();
    
    let v = vec![
        "hi",
        "I am",
        "from",
        "thread"
    ];

    let handle = thread::spawn(move || {
        for i in v {
            transmitter.send(i).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    // receiver 此处作为一个迭代器，不断其中元素
    // 
    for i in receiver {
        println!("i in receiver: {}", i);
    }
}
