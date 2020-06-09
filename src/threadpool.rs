use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

pub struct ThreadPool {
    threads: Vec<WorkerThread>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

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
        let job = Box::new(t);
        self.sender.send(job).unwrap();
    }
}

struct WorkerThread {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl WorkerThread {
    fn new(i: usize, r: Arc<Mutex<mpsc::Receiver<Job>>>) -> WorkerThread {
        let thread = thread::spawn( move || {
            while let Ok(job) = r.lock().unwrap().recv() {
                println!("Thread {} working...", i);
                job();
            }
        });

        WorkerThread {
            id: i,
            thread: thread,
        }
    }
}
