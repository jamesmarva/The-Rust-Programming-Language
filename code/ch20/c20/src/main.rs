use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn main() {

    let pool = ThreadPool::new(4);
    for i in 0..8 {
        println!("{}", i);
    }
    for i in 0..8 {
        thread::sleep(Duration::from_secs(1));

        pool.execute(move || {
            println!("{}", i);
        });
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker { 
        
        let thread = thread::spawn(move || 
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("Worker {} got a job: executing.", id);
                // job = FnOnce()
                job();
        });
        Worker{id, thread}
    }
}

struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {

    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool {workers, sender}
    }

    pub fn execute<F>(&self, f: F) 
    where 
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

