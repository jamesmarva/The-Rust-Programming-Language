use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");
}

type Job = Box<dyn FnOnce() + Send + 'static>;
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new (id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = std::thread::spawn(move || {
           loop {
               let job = receiver.lock().unwrap().recv().unwrap();
               job()
           } 
        });
        Worker{id, thread}
    }
}

struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    fn new (size: usize) -> ThreadPool {
        let mut workers  = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();
        let rece = Arc::new(Mutex::new(receiver));
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&rece)));
        }
        ThreadPool{workers, sender}
    }

    fn execute<T>(&self, t: T) 
    where 
        T: FnOnce() + Send + 'static,
    {
        let job = Box::new(t);
        self.sender.send(job).unwrap();
    }
}
impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            worker.thread.join().unwrap();
        }
    }
}


