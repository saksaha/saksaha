use super::DiscoveryTask;
use logger::{tdebug, terr};
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};
use task_queue::TaskQueue;

pub(crate) struct DiscTaskHandler {
    pub(crate) task_queue: Arc<TaskQueue<DiscoveryTask>>,
    pub(crate) task_min_interval: Duration,
}

impl DiscTaskHandler {
    pub(crate) fn new(
        task_queue: Arc<TaskQueue<DiscoveryTask>>,
        disc_task_interval: Option<u16>,
    ) -> DiscTaskHandler {
        let task_min_interval = match disc_task_interval {
            Some(i) => Duration::from_millis(i.into()),
            None => Duration::from_millis(1000),
        };

        DiscTaskHandler {
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

                let task = match task_queue.pop_front().await {
                    Ok(t) => {
                        tdebug!(
                            "p2p_discovery",
                            "task",
                            "Popped a task. Will handle, {:?}",
                            t,
                        );
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
