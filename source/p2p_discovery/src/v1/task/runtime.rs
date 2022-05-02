use super::{
    handler::Handler, task::DiscoveryTask, task::DiscoveryTaskInstance,
    task::TaskInstance, TaskResult,
};
use logger::{tdebug, terr};
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};
use task_queue::TaskQueue;

const MAX_TASK_RETRY: usize = 3;

pub(crate) struct DiscTaskRuntime {
    pub(crate) task_queue: Arc<TaskQueue<DiscoveryTaskInstance>>,
    pub(crate) task_min_interval: Duration,
}

impl DiscTaskRuntime {
    pub(crate) fn new(
        task_queue: Arc<TaskQueue<DiscoveryTaskInstance>>,
        disc_task_interval: Option<u16>,
    ) -> DiscTaskRuntime {
        let task_min_interval = match disc_task_interval {
            Some(i) => Duration::from_millis(i.into()),
            None => Duration::from_millis(1000),
        };

        DiscTaskRuntime {
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
                    TaskResult::FailRetry => {
                        if task_instance.fail_count < MAX_TASK_RETRY - 1 {
                            tdebug!(
                                "p2p_discovery",
                                "task",
                                "Task failed retriable, task: {}",
                                task_instance
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

async fn wait_until_min_interval(
    time_since: SystemTime,
    min_interval: Duration,
) {
    match time_since.elapsed() {
        Ok(d) => {
            if d < min_interval {
                let diff = min_interval - d;
                tokio::time::sleep(diff).await;
            }
        }
        Err(err) => {
            terr!(
                "p2p_discovery",
                "task",
                "Calculating the time elapsed fail, err: {}",
                err
            );

            tokio::time::sleep(min_interval).await;
        }
    }
}
