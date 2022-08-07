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

pub type Handler<T> =
    Box<dyn Fn(T) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

pub struct TaskQueue2<T>
where
    T: Send + Sync,
{
    tx: Sender<T>,
    // rx: Receiver<T>,
}

impl<T> TaskQueue2<T>
where
    T: std::fmt::Display + Send + Sync + 'static,
{
    pub async fn init(
        capacity: usize,
        task_min_interval: Option<u16>,
        handler: Handler<T>,
    ) -> TaskQueue2<T> {
        let (tx, rx) = mpsc::channel(capacity);

        // let rx = Arc::new(Mutex::new(rx));
        let handler = Arc::new(handler);

        tokio::spawn(async move {
            let runtime = TaskRuntime2::new(
                rx,
                // rx.clone(),
                task_min_interval,
                handler.clone(),
            );

            runtime.run().await;
        });

        let task_queue = TaskQueue2 {
            tx,
            // rx,
            // rx: Mutex::new(rx),
        };

        task_queue
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

    // pub async fn pop_front(&self) -> Result<T, String> {
    //     let mut rx = self.rx.lock().await;

    //     match rx.recv().await {
    //         Some(t) => return Ok(t),
    //         None => {
    //             return Err(format!(
    //                 "Task queue is already closed. \
    //                 Something might have gone wrong",
    //             ));
    //         }
    //     }
    // }
}

pub(crate) struct TaskRuntime2<T> {
    pub task_rx: Receiver<T>,
    pub task_min_interval: Duration,
    pub handler: Arc<Handler<T>>,
}

impl<T> TaskRuntime2<T>
where
    T: std::fmt::Display, // + Send + Sync + 'static,
{
    pub fn new(
        // task_queue: Arc<TaskQueue2<T>>,
        task_rx: Receiver<T>,
        disc_task_interval: Option<u16>,
        handler: Arc<Handler<T>>,
    ) -> TaskRuntime2<T> {
        let task_min_interval = match disc_task_interval {
            Some(i) => Duration::from_millis(i.into()),
            None => Duration::from_millis(TASK_MIN_INTERVAL),
        };

        TaskRuntime2 {
            // task_queue,
            task_rx,
            task_min_interval,
            handler,
        }
    }

    pub(crate) async fn run(self) {
        let task_min_interval = &self.task_min_interval;
        // let task_queue = &self.task_queue;
        let mut task_rx = self.task_rx;

        loop {
            let time_since = SystemTime::now();

            let task = match task_rx.recv().await {
                Some(t) => {
                    debug!("Pop P2PTask - {}", t,);

                    t
                }
                None => {
                    error!(
                        "Cannot receive tasks any more, channel \
                        might have been closed,",
                    );
                    return;
                }
            };

            (self.handler)(task).await;

            sak_utils_time::wait_until_min_interval(
                time_since,
                *task_min_interval,
            )
            .await;
        }
    }
}
