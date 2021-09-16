use crate::{common::errors::Error, err_res};
use logger::log;
use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

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

            match job {
                Ok(job) => {
                    let job = match job.recv() {
                        Ok(j) => j,
                        Err(err) => {
                            log!(DEBUG, "Error receiving job, err: {}\n", err);
                            panic!();
                        }
                    };

                    println!("Worker {} got a job; executing.\n", id);

                    job();
                }
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
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Ok(ThreadPool { workers, sender })
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}
