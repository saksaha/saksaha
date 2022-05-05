use super::{handler, P2PTask};
use logger::{tdebug, terr};
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};
use task_queue::TaskQueue;

const TASK_MIN_INTERVAL: u64 = 1000;

pub(crate) struct P2PTaskRuntime {
    pub(crate) task_queue: Arc<TaskQueue<P2PTask>>,
    pub(crate) task_min_interval: Duration,
}

impl P2PTaskRuntime {
    pub(crate) fn new(
        task_queue: Arc<TaskQueue<P2PTask>>,
        disc_task_interval: Option<u16>,
    ) -> P2PTaskRuntime {
        let task_min_interval = match disc_task_interval {
            Some(i) => Duration::from_millis(i.into()),
            None => Duration::from_millis(TASK_MIN_INTERVAL),
        };

        P2PTaskRuntime {
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
                    tdebug!(
                        "saksaha",
                        "p2p",
                        "Popped a task. Will handle, {}",
                        t,
                    );
                    t
                }
                Err(err) => {
                    terr!(
                        "saksaha",
                        "p2p",
                        "Cannot handle p2p discovery task any more, \
                                err: {}",
                        err,
                    );
                    return;
                }
            };

            // let handler = Handler { task };
            // handler.run().await;
            handler::run(task).await;

            utils_time::wait_until_min_interval(time_since, *task_min_interval)
                .await;
        }
    }
}
