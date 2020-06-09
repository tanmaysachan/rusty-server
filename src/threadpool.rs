use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(mut size: usize) -> ThreadPool {
        // use 1 thread if size out of bounds
        if size < 1 || size > 4 {
            size = 1;
        }

        println!("Spinning up {} threads...\n", size);

        let mut pool = Vec::with_capacity(size);

        for i in 0..size {
            
        }

        ThreadPool{
            threads: pool,
        }
    }

    pub fn run<T>(&self, t: T)
    where
        T: FnOnce() + Send + 'static,
    {
        
    }
}
