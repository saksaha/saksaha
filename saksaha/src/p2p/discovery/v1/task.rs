use crate::{
    common::{Error, Result},
    err,
};
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
    pub make_action: Box<dyn Fn() -> BoxedFuture + Send + Sync>,
    pub fail_count: usize,
}

impl Task {
    pub fn new<F>(make_action: F) -> Task
    where
        F: Fn() -> BoxedFuture + Send + Sync + 'static,
    {
        let make_action = Box::new(make_action);
        Task {
            make_action,
            fail_count: 0,
        }
    }
}

pub struct TaskQueue {
    tx: Arc<Sender<Task>>,
    rx: Arc<Mutex<Receiver<Task>>>,
    max_retry: usize,
    interval: Duration,
}

impl TaskQueue {
    pub fn new() -> TaskQueue {
        let (tx, rx) = mpsc::channel(10);

        TaskQueue {
            tx: Arc::new(tx),
            rx: Arc::new(Mutex::new(rx)),
            max_retry: 2,
            interval: Duration::from_millis(1000),
        }
    }

    pub async fn push(&self, task: Task) -> Result<()> {
        return TaskQueue::_push(self.tx.clone(), task).await;
    }

    async fn _push(tx: Arc<Sender<Task>>, task: Task) -> Result<()> {
        match tx.send(task).await {
            Ok(_) => Ok(()),
            Err(err) => return err!("Cannot enqueue new task, err: {}", err),
        }
    }

    pub fn run_loop(&self) {
        let rx = self.rx.clone();
        let tx = self.tx.clone();
        let max_retry = self.max_retry;
        let interval = self.interval;

        tokio::spawn(async move {
            let mut rx = rx.lock().await;

            loop {
                if let Some(t) = rx.recv().await {
                    match (&t.make_action)().await {
                        TaskResult::Success => {}
                        TaskResult::Retriable => {
                            if t.fail_count < max_retry {
                                tokio::time::sleep(interval).await;

                                let t = Task {
                                    make_action: t.make_action,
                                    fail_count: t.fail_count + 1,
                                };

                                if let Err(err) =
                                    TaskQueue::_push(tx.clone(), t).await
                                {
                                    log!(DEBUG, "Fatal error, {}\n", err);
                                }
                            }
                        }
                        TaskResult::Fail(err) => {
                            log!(
                                DEBUG,
                                "Unexpected error while \
                                executing a task, err: {}",
                                err
                            );
                        }
                    };
                }
            }
        });
    }
}

macro_rules! task {
    (async $d:tt) => {
        {
            let t = $crate::p2p::discovery::task::Task::new(
                || Box::pin(async $d));
            t
        }
    };

    (async move $d:tt) => {
        {
            let t = $crate::p2p::discovery::task::Task::new(
                move || Box::pin(async move $d));
            t
        }
    };
}

pub(crate) use task;
