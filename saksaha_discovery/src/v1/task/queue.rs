use log::{debug};
use std::{sync::Arc, time::Duration};
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex,
};
use crate::{task::TaskKind};

use super::{Task, TaskError};

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

    pub async fn push(&self, task_kind: TaskKind) -> Result<(), TaskError> {
        let t = Task {
            kind: task_kind,
            fail_count: 0,
        };

        match self.tx.send(t).await {
            Ok(_) => Ok(()),
            Err(err) => {
                let msg = format!("Cannot enqueue new task, err: {}", err);
                return Err(TaskError::Default(msg));
            }
        }
    }

    async fn execute_task(t: Task) {
        let task_result = match t.kind {
            // TaskKind::Ping(addr) => {
            //     PingPong::ping(addr).await
            // }
        };
    }

    pub fn run_loop(&self) {
        let rx = self.rx.clone();
        let tx = self.tx.clone();
        let max_retry = self.max_retry;
        let interval = self.interval;

        tokio::spawn(async move {
            let mut rx = rx.lock().await;

            loop {
                let task = match rx.recv().await {
                    Some(t) => t,
                    None => {
                        debug!("Cannot receive task any more");
                        break;
                    }
                };

                TaskQueue::execute_task(task).await;

            }
        });
    }
}
