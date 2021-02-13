fn main() {
    println!("Hello, world!");
}


use std::thread;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        let thread = std::thread::spawn(|| {});
        Worker{id, thread}
    }
}
pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {

    pub fn new (size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id));
        }
        ThreadPool{workers}
    }

    
}