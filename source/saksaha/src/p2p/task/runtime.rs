use super::task::{P2PTaskInstance, TaskResult};
use super::{handler::Handler, task::TaskInstance};
use logger::{tdebug, terr};
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};
use task_queue::TaskQueue;
use utils_time::wait_until_min_interval;

const MAX_TASK_RETRY: usize = 2;

pub(crate) struct P2PTaskRuntime {
    pub(crate) task_queue: Arc<TaskQueue<P2PTaskInstance>>,
    pub(crate) task_min_interval: Duration,
}

impl P2PTaskRuntime {
    pub(crate) fn new(
        task_queue: Arc<TaskQueue<P2PTaskInstance>>,
        disc_task_interval: Option<u16>,
    ) -> P2PTaskRuntime {
        let task_min_interval = match disc_task_interval {
            Some(i) => Duration::from_millis(i.into()),
            None => Duration::from_millis(1000),
        };

        P2PTaskRuntime {
            task_queue,
            task_min_interval,
        }
    }

    pub(crate) fn run(&self) {
        let task_queue = self.task_queue.clone();
        let task_min_interval = self.task_min_interval.clone();

        tokio::spawn(async move {
            loop {
                let time_since = SystemTime::now();

                let task_instance = match task_queue.pop_front().await {
                    Ok(t) => {
                        tdebug!(
                            "p2p_discovery",
                            "task",
                            "Popped a task. Will handle, {}",
                            t,
                        );
                        t
                    }
                    Err(err) => {
                        terr!(
                            "p2p_discovery",
                            "task",
                            "Cannot handle p2p discovery task any more, \
                                err: {}",
                            err,
                        );
                        return;
                    }
                };

                let handler = Handler {
                    task_instance: task_instance.clone(),
                };

                match handler.run().await {
                    TaskResult::Success => (),
                    TaskResult::Fail => (),
                    TaskResult::FailRetry { msg } => {
                        if task_instance.fail_count < MAX_TASK_RETRY {
                            tdebug!(
                                "p2p_discovery",
                                "task",
                                "Task failed retriable, task: {}, msg: {}",
                                task_instance,
                                msg,
                            );

                            let task_instance = TaskInstance {
                                task: task_instance.task.clone(),
                                fail_count: task_instance.fail_count + 1,
                            };

                            let _ = task_queue.push_back(task_instance).await;
                        }
                    }
                };

                wait_until_min_interval(time_since, task_min_interval).await;
            }
        });
    }
}
