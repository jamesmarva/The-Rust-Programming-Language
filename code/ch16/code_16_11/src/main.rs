use std::thread;
use std::sync::mpsc;
use std::time::Duration;

// 通过克隆发送者来创建多个生产者
fn main() {
    let (transmitter, receiver) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&transmitter);
    thread::spawn(move || {
        let vals = vec![
            "I",
            "am",
            "from",
            "first"
        ];
        for i in vals {
            transmitter.send(i).unwrap();
            thread::sleep(Duration::from_millis(800));
        }
    });

    thread::spawn(move|| {
        let vals = vec![
            "second",
            "spawn",
            "is",
            "me"
        ];
        for i in vals {
            tx1.send(i).unwrap();
            thread::sleep(Duration::from_millis(800));
        }
    });

    for r in receiver {
        println!("r is {} ", r);
    }
}
