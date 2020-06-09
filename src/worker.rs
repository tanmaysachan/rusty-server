use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

pub type Task = Box<dyn Send + FnOnce() + 'static>;

pub enum Signal {
    RUN(Task),
    TERM,
}

pub struct WorkerThread {
    pub id: usize,
    pub thread: Option<thread::JoinHandle<()>>,
}

impl WorkerThread {
    pub fn new(i: usize, r: Arc<Mutex<mpsc::Receiver<Signal>>>) -> WorkerThread {
        let thread = thread::spawn( move || {
            while let Ok(job) = r.lock().unwrap().recv() {
                match job {
                    Signal::RUN(task) => {
                        println!("Thread {} running...", i);
                        task();
                    }
                    Signal::TERM => {
                        println!("Terminating thread {}...", i);
                        break;
                    }
                }
            }
        });

        WorkerThread {
            id: i,
            thread: Some(thread),
        }
    }
}
