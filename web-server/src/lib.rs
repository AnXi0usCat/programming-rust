use std::thread;


pub struct ThreadPool {
    workers: Vec<Worker>
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        for i in 0..size {
            // initialize our threads
            workers.push(Worker::new(i))
        }

        ThreadPool { workers }
    }

    pub fn execute<F>(&self, f: F)
    where F: FnOnce() -> () + Send + 'static {

    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}

impl Worker {

    fn new(id: usize) -> Self {
        let thread = thread::spawn(|| {});

        Worker{ id, thread }
    }

}