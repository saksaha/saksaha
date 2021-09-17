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

type Job = Box<dyn FnOnce(usize) + Send + 'static>;

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

            job(id);
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
        F: FnOnce(usize) + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn it_needs_to_handle_many_requests() {
        let tpool =
        super::ThreadPool::new(5).expect("Thread pool needs to be created");

        // struct S {
        //     pub val: i32,
        // }

        // let a = S {
        //     val: 0,
        // };

        let a = 3;
        let a = super::Arc::new(a);

        for i in 0..20 {
            let v = super::Arc::clone(&a);

            tpool.execute(move |id| {
                println!("33 id: {}, v: {}", id, v);
                // std::thread::sleep(std::time::Duration::from_millis(3000));

                println!("44 id: {}, v: {}", id, v);
            });
        }
    }
}
