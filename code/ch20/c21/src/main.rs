use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn main() {

}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,  
    thread: thread::JoinHandle<()>
}


impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // An alternative implementation of Worker::new using while let
        let thread = thread::spawn(move|| {
            while let Ok(job) = receiver.lock().unwrap().recv() {
                println!("Worker {} got a job; executing.", id);
                job();
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
    pub fn new (size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool{workers, sender}
    }

    pub fn execute<T>(&self, t: T) 
    where 
        T: FnOnce() + Send + 'static,
    {
        let job = Box::new(t);
        self.sender.send(job).unwrap();
    }
}
