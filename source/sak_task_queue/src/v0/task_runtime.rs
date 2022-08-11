use crate::TaskQueue;
use async_trait::async_trait;
use futures::Future;
use log::{debug, error};
use std::{
    pin::Pin,
    sync::Arc,
    time::{Duration, SystemTime},
};

const TASK_MIN_INTERVAL: u64 = 1000;

pub type HandlerFn<T, C> =
    Box<dyn Fn(T, &C) -> Pin<Box<dyn Future<Output = ()> + Send + Sync>>>;

pub struct TaskRuntime<T, C>
where
    T: std::fmt::Display + Send + Sync + 'static,
    C: Send + Sync,
{
    task_queue: Arc<TaskQueue<T>>,
    task_min_interval: Duration,
    ctx: C,
    handler_fn: HandlerFn<T, C>,
}

impl<T, C> TaskRuntime<T, C>
where
    T: std::fmt::Display + Send + Sync + 'static,
    C: Send + Sync,
{
    pub fn new(
        task_queue: Arc<TaskQueue<T>>,
        task_min_interval: Option<u64>,
        ctx: C,
        handler_fn: HandlerFn<T, C>,
    ) -> TaskRuntime<T, C> {
        let task_min_interval = match task_min_interval {
            Some(i) => Duration::from_millis(i.into()),
            None => Duration::from_millis(TASK_MIN_INTERVAL),
        };

        TaskRuntime {
            task_queue,
            task_min_interval,
            ctx,
            handler_fn,
        }
    }

    pub async fn run(self) {
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

            (self.handler_fn)(task, &self.ctx).await;

            sak_utils_time::wait_until_min_interval(
                time_since,
                *task_min_interval,
            )
            .await;
        }
    }
}

#[async_trait]
pub trait HandleTask<T>
where
    T: std::fmt::Display + Send + Sync + 'static,
{
    pub async fn handle_task(&self) {}
}
