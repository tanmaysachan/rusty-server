use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

use crate::worker::Signal;
use crate::worker::WorkerThread;

pub struct ThreadPool {
    threads: Vec<WorkerThread>,
    sender: mpsc::Sender<Signal>,
}


impl ThreadPool {
    pub fn new(mut size: usize) -> ThreadPool {
        // use 1 thread if size out of bounds
        if size < 1 || size > 4 {
            size = 1;
        }

        println!("Spinning up {} threads...\n", size);

        let (s, r) = mpsc::channel();

        let r = Arc::new(Mutex::new(r));

        let mut pool = Vec::with_capacity(size);

        for i in 0..size {
            pool.push(WorkerThread::new(i, Arc::clone(&r)));
        }

        ThreadPool {
            threads: pool,
            sender: s,
        }
    }

    pub fn run<T>(&self, t: T)
    where
        T: FnOnce() + Send + 'static,
    {
        let signal = Signal::RUN(Box::new(t));
        if let Err(e) = self.sender.send(signal) {
            eprintln!("Channel error: {}", e);
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.threads {
            if let Err(e) = self.sender.send(Signal::TERM) {
                eprintln!("ThreadPool error: {}", e)
            }
        }
        for worker in &mut self.threads {
            println!("Shutting down thread {}...", worker.id);
            if let Some(thread) = worker.thread.take() {
                if let Ok(_) = thread.join() {
                    println!("Shutdown thread {}", worker.id);
                }
            }
        }
    }
}
