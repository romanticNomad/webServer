pub struct ThreadPool;

impl ThreadPool {
    pub fn new(num: usize) -> ThreadPool {
        ThreadPool
    }

    pub fn execute<F>(&self, f: F)
    where
        F: Send + FnOnce() + 'static,
        {
            
        }
}