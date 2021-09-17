use crate::{common::errors::Error, err_res};
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
    pub fn new(size: usize) -> Result<ThreadPool, Error> {
        assert!(size > 0);
        if size < 1 {
            return err_res!("Size must be greater than 0");
        }

        let (sender, receiver) = mpsc::channel();

        let receiver: Arc<Mutex<mpsc::Receiver<Job>>> = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Ok(ThreadPool { workers, sender })
    }

    pub fn stop(&self) {

    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce(usize) -> Option<bool> + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

use std::{sync, time};
use std::sync::atomic::{AtomicBool, Ordering};

pub struct Timer {
    handle: Option<thread::JoinHandle<()>>,
    alive: sync::Arc<AtomicBool>,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            handle: None,
            alive: sync::Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn start<F>(&mut self, fun: F)
        where F: 'static + Send + FnMut() -> ()
    {
        self.alive.store(true, Ordering::SeqCst);

        let alive = self.alive.clone();

        self.handle = Some(thread::spawn(move || {
            let mut fun = fun;
            while alive.load(Ordering::SeqCst) {
                fun();
                thread::sleep(time::Duration::from_millis(10));
            }
        }));
    }

    pub fn stop(&mut self) {
        self.alive.store(false, Ordering::SeqCst);
        self.handle
            .take().expect("Called stop on non-running thread")
            .join().expect("Could not join spawned thread");
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

        println!("t");

        let tpool = ThreadPool::new(5).unwrap();

        println!("t2");

        for i in 0..10 {
            tpool.execute(move |id| {
                println!("33 id: {}, v: {}", id, i);

                std::thread::sleep(std::time::Duration::from_millis(500));

                println!("44 id: {}, v: {}", id, i);

                Some(true)
            });
        }

        // tpool.workers;

        // let b: Vec<Box<thread::JoinHandle<()>>> = tpool
        //     .workers
        //     .into_iter()
        //     .map(|v| Box::new(v.thread))
        //     .collect();

        // b.into_iter().for_each(|h| {
        //     h.join().unwrap();
        // });
    }
}
