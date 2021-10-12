use crate::{
    common::{Error, Result},
    err,
};
use futures::future::BoxFuture;
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
    pub action: Pin<Box<dyn Future<Output = TaskResult<Error>> + Send + Sync>>,
    pub fail_count: usize,
}

impl Task {
    pub fn new<F>(action: F) -> Task
    where
        F: Future<Output = TaskResult<Error>> + Send + Sync + 'static,
    {
        let action = Box::pin(action);
        Task {
            action,
            // make_action,
            fail_count: 0,
        }
    }
}

pub struct TaskQueue {
    tx: Arc<Sender<Box<dyn FnOnce() -> Task + Send + Sync>>>,
    rx: Arc<Mutex<Receiver<Box<dyn FnOnce() -> Task + Send + Sync>>>>,
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

    // pub async fn push(&self, task: Task) -> Result<()> {
    //     return TaskQueue::_push(self.tx.clone(), task).await;
    // }


    pub async fn push(&self, task: Task) -> Result<()> {
        // let make_task = Box::new(|| task);
        return TaskQueue::_push(self.tx.clone(), task).await;
    }

    async fn _push(
        tx: Arc<Sender<Box<dyn FnOnce() -> Task + Send + Sync>>>,
        task:fe
        // task: Box<dyn FnOnce() -> Task + Send + Sync>,
    ) -> Result<()> {
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
                if let Some(make_task) = rx.recv().await {
                    let t = make_task();
                    // let t2 = make_task();
                    // let action = (t.make_action)();
                    // let new_make_action = Box::new(|| action);

                    // match (t.make_action)().await {
                    //     TaskResult::Success => {}
                    //     TaskResult::Retriable => {
                    //         if t.fail_count < max_retry {
                    //             tokio::time::sleep(interval).await;

                    //             let t = Task {
                    //                 // make_action: new_make_action,
                    //                 fail_count: t.fail_count + 1,
                    //             };

                    //             if let Err(err) =
                    //                 TaskQueue::_push(tx.clone(), t).await
                    //             {
                    //                 log!(DEBUG, "Fatal error, {}\n", err);
                    //             }
                    //         }
                    //     }
                    //     TaskResult::Fail(err) => {
                    //         log!(
                    //             DEBUG,
                    //             "Unexpected error while \
                    //             executing a task, err: {}",
                    //             err
                    //         );
                    //     }
                    // };
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
                || Box::pin(async move $d));
            t
        }
    };
}

pub(crate) use task;
