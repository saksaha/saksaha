use crate::common::{Error, Result};
use logger::log;
use std::{future::Future, pin::Pin, sync::Arc, time::Duration};
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex,
};

// type BoxedFuture = Box<dyn Fn() -> Pin<Box<dyn Future<Output = Result<()>> + Send>> + Send + Sync>;
type BoxedFuture =
    Pin<Box<dyn Future<Output = TaskResult<Error>> + Send + Sync>>;

pub enum TaskResult<E> {
    Success,

    Retriable,

    Fail(E),
}

pub struct Task {
    pub f: Box<dyn Fn() -> BoxedFuture + Send + Sync>,
    pub fail_count: usize,
}

impl Task {
    pub fn new<F>(f: F) -> Task
    where
        F: Fn() -> BoxedFuture + Send + Sync + 'static,
    {
        let a = Box::new(f);
        Task { f: a, fail_count: 0 }
    }
}

pub struct TaskQueue {
    tx: Arc<Sender<Task>>,
    rx: Arc<Mutex<Receiver<Task>>>,
    max_retry: usize,
}

impl TaskQueue {
    pub fn new() -> TaskQueue {
        let (tx, mut rx) = mpsc::channel(10);

        TaskQueue {
            tx: Arc::new(tx),
            rx: Arc::new(Mutex::new(rx)),
            max_retry: 3,
        }
    }

    pub async fn push(&self, task: Task) {
        self.tx.send(task).await;
    }

    pub fn run_loop(&self) {
        let rx = self.rx.clone();
        let tx = self.tx.clone();
        let max_retry = self.max_retry;

        tokio::spawn(async move {
            println!("11");
            let mut rx = rx.lock().await;

            loop {
                if let Some(t) = rx.recv().await {
                    println!("fail_count: {}", t.fail_count);

                    let f = &t.f;
                    match f().await {
                        TaskResult::Success => {
                        }
                        TaskResult::Retriable => {
                            println!("1313");
                            if t.fail_count < max_retry {
                                let t = Task {
                                    f: t.f,
                                    fail_count: t.fail_count + 1,
                                };

                                println!("555");

                                tokio::time::sleep(Duration::from_millis(1000))
                                    .await;
                                tx.send(t).await;
                            }
                        }
                        TaskResult::Fail(err) => {
                            log!(DEBUG, "Unexpected error while executing a task, err: {}", err);
                        }
                    };
                }
            }
        });
    }
}

macro_rules! task {
    (async $d:tt) => {
        || {
            let t = $crate::p2p::discovery::task::Task::new(Box::pin(async $d));
            t
        }
    };

    (async move $d:tt) => {
        || {
            let t = $crate::p2p::discovery::task::Task::new(Box::pin(async move $d));
            t
        }
    };
}

pub(crate) use task;
