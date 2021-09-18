use crate::{common::SakResult, err_res};
use logger::log;
use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    pub workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

pub struct Worker {
    id: usize,
    pub thread: thread::JoinHandle<()>,
}

type Job = Box<dyn FnOnce(usize) -> Option<bool> + Send + 'static>;

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let guard = match receiver.lock() {
                Ok(j) => j,
                Err(err) => {
                    log!(
                        DEBUG,
                        "Error acquiring the mutex, tid: {}, err: {}\n",
                        id,
                        err
                    );
                    continue;
                }
            };

            let job = match guard.recv() {
                Ok(j) => j,
                Err(err) => {
                    log!(
                        DEBUG,
                        "Error receiving the job, tid: {}, err: {}\n",
                        id,
                        err
                    );
                    continue;
                }
            };

            std::mem::drop(guard);

            println!("Worker {} got a job; executing.", id);

            if let Some(b) = job(id) {
                if b {
                    return;
                }
            }
        });

        Worker { id, thread }
    }
}

impl ThreadPool {
    pub fn new(size: usize) -> SakResult<ThreadPool> {
        assert!(size > 0);
        if size < 1 {
            return err_res!("Size must be greater than 0");
        }

        let (sender, receiver) = mpsc::channel();

        let receiver: Arc<Mutex<mpsc::Receiver<Job>>> =
            Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        println!("workers count: {}", size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        let a = async {

        };

        Ok(ThreadPool {
            workers: workers,
            sender,
        })
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce(usize) -> Option<bool> + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }

    pub fn join(tp: ThreadPool) -> SakResult<bool> {
        for w in tp.workers {
            // w.thread.join();
        }
        Ok(true)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn it_needs_to_handle_many_requests() {
        use super::ThreadPool;
        use rand::prelude::*;
        use std::thread;
        use std::time::Duration;

        println!("Start");

        let tpool = ThreadPool::new(5).unwrap();

        for i in 0..10 {
            println!("for, {}", i);

            tpool.execute(move |id| {
                println!("33 id: {}, v: {}", id, i);

                std::thread::sleep(std::time::Duration::from_millis(2000));

                println!("44 id: {}, v: {}", id, i);

                Some(true)
            });
        }

        let handles: Vec<Box<thread::JoinHandle<()>>> = tpool
            .workers
            .into_iter()
            .map(|v| Box::new(v.thread))
            .collect();

        for h in handles {
            h.join().unwrap();
        }

        std::thread::sleep(std::time::Duration::from_millis(20000));
    }
}
