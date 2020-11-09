use std::thread;
use std::sync::mpsc;

fn main() {

    let (transmitter, recevier) = mpsc::channel();
    let handle = thread::spawn(move || {

        // 返回一个result 类型，要进行处理错误
        transmitter.send("asdfasfdasdfsad").unwrap();
    });

    println!("{}",recevier.recv().unwrap());
}
