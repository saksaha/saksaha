use log::{debug, error};
use std::{sync::Arc, time::Duration};
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex,
};

use crate::identity::Identity;

use super::{
    address::Address, ops::whoareyou::initiator::WhoAreYouInitiator,
    table::Table,
};

pub enum TaskError {
    Default(String),
}

#[derive(Clone)]
pub enum Task {
    InitiateWhoAreYou(Arc<Table>, Address),
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
    table: Arc<Table>,
    a: Mutex<Option<u16>>,
    // id: Arc<impl Identity + 'static>,
}

impl TaskQueue {
    pub fn new(
        table: Arc<Table>,
        id: Arc<Box<dyn Identity>>,
    ) -> TaskQueue {
        let (tx, rx) = mpsc::channel(10);

        TaskQueue {
            tx: Arc::new(tx),
            rx: Arc::new(Mutex::new(rx)),
            max_retry: 2,
            interval: Duration::from_millis(1000),
            table,
            a: Mutex::new(None),
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

    pub async fn set(&self) {
        let mut a = self.a.lock().await;
        *a = Some(10);
        // self.a = ;
    }

    pub fn run_loop(&self) {
        let rx = self.rx.clone();
        let tx = self.tx.clone();
        let table = self.table.clone();

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

                match TaskRunner::run(table.clone(), &mut task_instance).await {
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

struct TaskRunner;

impl TaskRunner {
    pub async fn run(
        table: Arc<Table>,
        task_instance: &mut TaskInstance,
    ) -> Result<(), TaskError> {
        let task_result: Result<(), String> = match &task_instance.task {
            Task::InitiateWhoAreYou(table, addr) => {
                WhoAreYouInitiator::run(table.clone(), addr).await;

                Ok(())
                // PingPong::ping(addr).await
            }
        };

        match task_result {
            Ok(_) => (),
            Err(err) => {
                task_instance.fail_count += 1;
            }
        };

        Ok(())
    }
}
