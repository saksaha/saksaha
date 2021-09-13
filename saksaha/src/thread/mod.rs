use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};
use crate::{
    common::errors::Error,
    err_res,
};
use logger::log;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock();

            println!("{}", 444);

            match job {
                Ok(job) => {
                    let job = job.recv().unwrap();
                    println!("Worker {} got a job; executing.\n", id);
                    job();
                },
                Err(err) => {
                    log!(DEBUG, "Error getting a job, err: {}\n", err);
                    panic!("33")
                }
            }
        });

        Worker { id, thread }
    }
}

impl ThreadPool {
    pub fn new(size: usize) -> Result<ThreadPool, Error> {
        assert!(size > 0);
        if size < 1 {
            return err_res!("Size must be greater than 0");
        }

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            println!("{}", id);
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Ok(ThreadPool { workers, sender })
    }
}
