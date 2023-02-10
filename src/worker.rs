use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::thread;

use log::info;

/// Type alias to represent any closure that will be passed on to the worker
/// through the receiver by the `ThreadPool`.
pub type Job = Box<dyn FnOnce() + Send + 'static>;

/// Worker responsible to consume jobs from the main thread.
pub struct Worker {
    /// Unique ID of the worker.
    pub id: usize,
    /// Thread spawned by the worker to consume a `Job`.
    pub thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    /// Creates a new instance by spawning a thread.
    /// The thread runs on an infinite loop querying the 
    /// receiver channel for jobs.
    pub fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    info!("Worker {} executing job.", id);
                    job();
                }
                Err(_) => {
                    info!("Worker {} terminated.", id);
                    break;
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}
