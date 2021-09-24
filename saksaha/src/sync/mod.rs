use std::sync::{Once, };

use tokio::sync::Mutex;

pub struct Sync {
    task_count: Mutex<usize>,
}

impl Sync {
    pub fn new() -> Sync {
        let task_count = Mutex::new(0);
        Sync {
            task_count,
        }
    }
}

impl Sync {
    pub fn spawn<T>(&self, future: T) -> tokio::task::JoinHandle<T::Output>
    where
        T: std::future::Future + Send + 'static,
        T::Output: Send + 'static,
    {
        // let task_count = self.task_count.lock().await;
        // *task_count = 1;

        // tokio::spawn(future)
    }
}
