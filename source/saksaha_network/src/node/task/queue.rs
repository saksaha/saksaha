use std::pin::Pin;

use futures::Future;
use sak_task_queue::TaskQueue2;

use super::NodeTask;

pub(in crate::node) struct NodeTaskQueue {}

impl NodeTaskQueue {
    pub async fn init() -> NodeTaskQueue {
        let handler_wrap: Box<
            dyn Fn(NodeTask) -> Pin<Box<dyn Future<Output = ()>>>,
        > = Box::new(|task: NodeTask| Box::pin(handler(task)));

        let t = NodeTask::Hello;

        let task_queue: TaskQueue2<NodeTask> =
            TaskQueue2::init(10, handler_wrap).await;

        NodeTaskQueue {}
    }
}

async fn handler(task: NodeTask) {
    // Box::new(|| Box::pin(async {}))

    // dyn Fn() -> Pin<Box<dyn Future<Output = ()> + Send + Sync>>
    //     + Send
    //     + Sync,
}
