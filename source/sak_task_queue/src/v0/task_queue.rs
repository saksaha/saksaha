use std::sync::Arc;
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex,
};

pub struct TaskQueue<T>
where
    T: std::fmt::Display + Send + Sync + 'static,
{
    tx: Arc<Sender<T>>,
    rx: Mutex<Receiver<T>>,
}

impl<T> TaskQueue<T>
where
    T: std::fmt::Display + Send + Sync + 'static,
{
    pub fn new(capacity: usize) -> TaskQueue<T> {
        let (tx, rx) = mpsc::channel(capacity);

        let tx = Arc::new(tx);

        TaskQueue {
            tx,
            rx: Mutex::new(rx),
        }
    }

    pub fn new_pusher(&self) -> TaskPusher<T> {
        let tx = self.tx.clone();

        TaskPusher::new(tx)
    }

    pub async fn push_back(&self, task: T) -> Result<(), String> {
        let task_str = task.to_string();

        match self.tx.send(task).await {
            Ok(_) => return Ok(()),
            Err(err) => {
                return Err(format!(
                    "Cannot add a new task, task: {}, err: {}",
                    task_str, err,
                ));
            }
        };
    }

    pub async fn pop_front(&self) -> Result<T, String> {
        let mut rx = self.rx.lock().await;

        rx.recv().await.ok_or(format!(
            "Cannot receive tasks any more. Task queue is closed.",
        ))
    }
}

pub struct TaskPusher<T>
where
    T: std::fmt::Display + Send + Sync + 'static,
{
    tx: Arc<Sender<T>>,
}

impl<T> TaskPusher<T>
where
    T: std::fmt::Display + Send + Sync + 'static,
{
    pub fn new(tx: Arc<Sender<T>>) -> TaskPusher<T> {
        TaskPusher { tx }
    }

    pub async fn push_back(&self, task: T) -> Result<(), String> {
        let task_str = task.to_string();

        match self.tx.send(task).await {
            Ok(_) => return Ok(()),
            Err(err) => {
                return Err(format!(
                    "Cannot add a new task, task: {}, err: {}",
                    task_str, err,
                ));
            }
        };
    }
}
