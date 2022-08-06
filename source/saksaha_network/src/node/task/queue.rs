use std::pin::Pin;

use futures::Future;
use sak_task_queue::{Handler, TaskQueue2};

use super::NodeTask;

pub(in crate::node) struct NodeTaskQueue {}

impl NodeTaskQueue {
    pub async fn init(task_min_interval: Option<u16>) -> NodeTaskQueue {
        let handler_wrap: Handler<NodeTask> =
            Box::new(|task: NodeTask| Box::pin(handler(task)));

        let t = NodeTask::Hello;

        let task_queue: TaskQueue2<NodeTask> =
            TaskQueue2::init(10, task_min_interval, handler_wrap).await;

        NodeTaskQueue {}
    }
}

async fn handler(task: NodeTask) {
    // Box::new(|| Box::pin(async {}))

    // dyn Fn() -> Pin<Box<dyn Future<Output = ()> + Send + Sync>>
    //     + Send
    //     + Sync,
}
