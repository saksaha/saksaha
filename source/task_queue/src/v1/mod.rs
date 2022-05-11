use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex,
};

pub struct TaskQueue<T>
where
    T: Send + Sync,
{
    tx: Sender<T>,
    rx: Mutex<Receiver<T>>,
}

impl<T> TaskQueue<T>
where
    T: std::fmt::Display + Send + Sync + 'static,
{
    pub fn new(capacity: usize) -> TaskQueue<T> {
        let (tx, rx) = mpsc::channel(capacity);

        TaskQueue {
            tx,
            rx: Mutex::new(rx),
        }
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
        match rx.recv().await {
            Some(t) => return Ok(t),
            None => {
                return Err(format!(
                    "Task queue is already closed. \
                    Something might have gone wrong",
                ));
            }
        }
    }
}
