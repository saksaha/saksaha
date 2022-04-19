use super::DiscoveryTask;
use logger::terr;
use std::sync::Arc;
use task_queue::TaskQueue;

pub(crate) struct TaskHandler {
    pub(crate) task_queue: Arc<TaskQueue<DiscoveryTask>>,
}

impl TaskHandler {
    pub(crate) fn run(&self) {
        let task_queue = self.task_queue.clone();

        tokio::spawn(async move {
            loop {
                let task = match task_queue.pop_front().await {
                    Ok(t) => t,
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

                println!("task, {:?}", task);
            }
        });
    }
}
