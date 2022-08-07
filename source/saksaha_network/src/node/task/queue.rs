use super::NodeTask;
use futures::Future;
use sak_task_queue::{Handler, TaskQueue2};
use std::pin::Pin;

pub(in crate::node) struct NodeTaskQueue {
    pub queue: TaskQueue2<NodeTask>,
}

impl NodeTaskQueue {
    pub async fn init(task_min_interval: Option<u16>) -> NodeTaskQueue {
        let handler_wrap: Handler<NodeTask> =
            Box::new(|task: NodeTask| Box::pin(handler(task)));

        let task_queue: TaskQueue2<NodeTask> =
            TaskQueue2::init(10, task_min_interval, handler_wrap).await;

        NodeTaskQueue { queue: task_queue }
    }
}

async fn handler(task: NodeTask) {
    // Box::new(|| Box::pin(async {}))

    // dyn Fn() -> Pin<Box<dyn Future<Output = ()> + Send + Sync>>
    //     + Send
    //     + Sync,
}
