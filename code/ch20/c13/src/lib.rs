use std::thread;

pub struct ThreadPool{

    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new (size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut threads = Vec::with_capacity(size);
        for _ in 0..size {

        }
        ThreadPool{threads}
    }

    fn execute<F, T>(f: F) -> thread::JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
    {
        thread::spawn(f)
    }
}
