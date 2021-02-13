pub struct ThreadPool;

impl ThreadPool {

    pub fn new (size: usize) -> ThreadPool {
        assert!(size > 0);
        ThreadPool
    }

    fn excute<F, T>(&self, f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
    {
        
    }
}