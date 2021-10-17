use std::{sync::Arc, time::Duration};

use tokio::sync::{Mutex, mpsc::{self, Receiver, Sender}};

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
        let t  = Task {
            kind: task_kind,
            fail_count: 0,
        };

        match self.tx.send(t).await {
            Ok(_) => Ok(()),
            Err(err) => return err!("Cannot enqueue new task, err: {}", err),
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
                let t = rx.recv();

                match t.await {
                    Some(t) => {
                        // let t = make_task();
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
                    None => {
                        break;
                    }
                }
            }
        });
    }
}
