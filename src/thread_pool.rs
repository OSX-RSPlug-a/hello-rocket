use std::sync::{
    mpsc::{self, Sender},
    Arc, Mutex,
};

use log::info;

use crate::worker::{Job, Worker};

/// Instance to keep track of the workers. 
pub struct ThreadPool {
    /// Vector of workers
    pub workers: Vec<Worker>,
    /// Channel to send jobs to the workers (or consumers).
    pub sender: Option<Sender<Job>>,
}

impl ThreadPool {
    /// Creates a new ThreadPool instance.
    /// 
    /// `num_workers` specifies the number of threads that need to be spawned.
    /// 
    /// # Panics
    /// When `num_workers` <= 0
    pub fn new(num_workers: usize) -> ThreadPool {
        assert!(num_workers > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(num_workers);
        for i in 0..num_workers {
            workers.push(Worker::new(i, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    /// Executes a specified function or a closure that implements the 
    /// traits `FnOnce` and `Send`.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    /// When a ThreadPool instance is dropped, the sender attribute is dropped.
    /// All of the associated workers are instructed to terminate by joining the threads.
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            info!("Shutting down worker {}.", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
