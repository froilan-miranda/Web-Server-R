use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    /*
    We chose usize as the type of the size parameter, because we know 
    that a negative number of threads doesn’t make any sense. We also 
    know we’ll use this 4 as the number of elements in a collection of 
    threads, which is what the usize type is for
    */
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // create some threads and store them in a vector
            workers.push(Worker::new(id));
        }
        ThreadPool { workers }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {

    }
}

pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});
        Worker { id, thread }
    }
}
