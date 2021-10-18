use std::{sync::Arc, time::Duration};
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex,
};
use logger::log;
use crate::{common::Result, err, p2p::discovery::task::TaskKind};
use super::Task;

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

    pub async fn push(&self, task_kind: TaskKind) -> Result<()> {
        let t = Task {
            kind: task_kind,
            fail_count: 0,
        };

        match self.tx.send(t).await {
            Ok(_) => Ok(()),
            Err(err) => return err!("Cannot enqueue new task, err: {}", err),
        }
    }

    fn execute_task(t: Task) {
        match t.kind {
            TaskKind::InitiateWhoAreYou(addr) => {

            }
        }
    }

    pub fn run_listen_loop(&self) {
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
                        log!(DEBUG, "Cannot receive task any more\n");
                        break;
                    }
                };

                TaskQueue::execute_task(task);


            }
        });
    }
}
