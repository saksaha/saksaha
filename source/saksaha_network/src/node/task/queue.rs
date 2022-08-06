use futures::Future;
use sak_task_queue::TaskQueue2;

pub(in crate::node) struct NodeTaskQueue {}

impl NodeTaskQueue {
    pub fn init(&self) -> NodeTaskQueue {
        let task_queue = TaskQueue2::init(10, handler_wrap);

        NodeTaskQueue {}
    }
}

async fn handler() {}

fn handler_wrap() -> impl Future<Output = ()> {
    handler()
}
