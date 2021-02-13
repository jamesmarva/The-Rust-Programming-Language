use std::sync::mpsc;
use std::thread;

struct Worker {
    id: u32,
    thread: thread::JoinHandle<()>,
}

///
/// 这个worker的作用就是不断的冲队列里面把任务拿出来，然后执行。
/// 这里是临时的，所以创建的用空的闭包（closure）
impl Worker {
    fn new(id: u32) -> Worker {
        let thread = thread::spawn(|| {});
        Worker{id, thread}
    }
}

struct Job;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: u32) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let mut workers = Vec::with_capacity(size as usize);   
        for id in 0..size {
            workers.push(Worker::new(id));
        }   
        ThreadPool{workers, sender}
    }
}
fn main() {
    println!("Hello, world!");
}
