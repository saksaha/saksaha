use futures::Future;
use log::{debug, error};
use std::{
    pin::Pin,
    sync::Arc,
    time::{Duration, SystemTime},
};
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex,
};

const TASK_MIN_INTERVAL: u64 = 1000;

pub struct TaskQueue2<T>
// where
//     T: Send + Sync,
{
    tx: Sender<T>,
    rx: Mutex<Receiver<T>>,
}

impl<T> TaskQueue2<T>
where
    T: std::fmt::Display, // + Send + Sync + 'static,
{
    pub async fn init(
        capacity: usize,
        handler: Box<
            dyn Fn() -> impl Future<Output = ()>,
            // Pin<Box<dyn Future<Output = ()> + Send + Sync>>
            //         + Send
            //         + Sync,
        >,
    ) -> TaskQueue2<T> {
        let (tx, rx) = mpsc::channel(capacity);

        TaskQueue2 {
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

pub struct TaskRuntime2<T> {
    pub(crate) task_queue: Arc<TaskQueue2<T>>,
    pub(crate) task_min_interval: Duration,
}

impl<T> TaskRuntime2<T>
where
    T: std::fmt::Display, // + Send + Sync + 'static,
{
    pub(crate) fn new(
        task_queue: Arc<TaskQueue2<T>>,
        disc_task_interval: Option<u16>,
    ) -> TaskRuntime2<T> {
        let task_min_interval = match disc_task_interval {
            Some(i) => Duration::from_millis(i.into()),
            None => Duration::from_millis(TASK_MIN_INTERVAL),
        };

        TaskRuntime2 {
            task_queue,
            task_min_interval,
        }
    }

    pub(crate) async fn run(&self) {
        let task_min_interval = &self.task_min_interval;
        let task_queue = &self.task_queue;

        loop {
            let time_since = SystemTime::now();

            let task = match task_queue.pop_front().await {
                Ok(t) => {
                    debug!("Pop P2PTask - {}", t,);

                    t
                }
                Err(err) => {
                    error!(
                        "Cannot handle p2p discovery task any more, \
                                err: {}",
                        err,
                    );
                    return;
                }
            };

            // handler::run(task).await;

            // sak_utils_time::wait_until_min_interval(
            //     time_since,
            //     *task_min_interval,
            // )
            // .await;
        }
    }
}
