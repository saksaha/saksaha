use log::{debug, error};
use std::{sync::Arc, time::Duration};
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex,
};

use super::address::Address;

pub enum TaskError {
    Default(String),
}

#[derive(Clone)]
pub enum Task {
    WhoAreYou(Address),
}

#[derive(Clone)]
struct TaskInstance {
    task: Task,
    fail_count: usize,
}

pub struct TaskQueue {
    tx: Arc<Sender<TaskInstance>>,
    rx: Arc<Mutex<Receiver<TaskInstance>>>,
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

    pub async fn push(&self, task: Task) -> Result<(), String> {
        let task_instance = TaskInstance {
            task,
            fail_count: 0,
        };

        match self.tx.send(task_instance).await {
            Ok(_) => return Ok(()),
            Err(err) => {
                return Err(format!("Cannot enqueue new task, err: {}", err));
            }
        };
    }

    async fn execute_task(
        task_instance: &mut TaskInstance,
    ) -> Result<(), TaskError> {
        let task_result: Result<(), String> = match task_instance.task {
            // TaskKind::Ping(addr) => {
            //     PingPong::ping(addr).await
            // }
            _ => Err("".to_string()),
        };

        match task_result {
            Ok(_) => (),
            Err(err) => {
                task_instance.fail_count += 1;
            }
        };

        Ok(())
    }

    pub fn run_loop(&self) {
        let rx = self.rx.clone();
        let tx = self.tx.clone();
        let max_retry = self.max_retry;
        let interval = self.interval;

        tokio::spawn(async move {
            let mut rx = rx.lock().await;

            loop {
                let mut task_instance = match rx.recv().await {
                    Some(t) => t,
                    None => {
                        debug!("Cannot receive task any more");
                        break;
                    }
                };

                if task_instance.fail_count >= max_retry {
                    continue;
                }

                match TaskQueue::execute_task(&mut task_instance).await {
                    Ok(_) => (),
                    Err(_) => {
                        let mut task_instance = task_instance.clone();
                        task_instance.fail_count += 1;

                        match tx.send(task_instance).await {
                            Ok(_) => (),
                            Err(err) => {
                                error!("Cannot enqueue new task, err: {}", err)
                            }
                        };
                    }
                };
            }
        });
    }
}
