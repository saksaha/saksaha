use super::{NodeTask, NodeTaskRuntimeCtx};
use crate::node::SaksahaNodeError;
use sak_task_queue::{HandlerFn, TaskQueue, TaskRuntime};
use std::{pin::Pin, sync::Arc};

pub(in crate::node) struct NodeTaskRuntime {
    task_runtime: TaskRuntime<NodeTask, NodeTaskRuntimeCtx>,
}

impl NodeTaskRuntime {
    pub fn new(
        node_task_queue_clone: Arc<TaskQueue<NodeTask>>,
        node_task_min_interval: Option<u64>,
        node_task_runtime_ctx: NodeTaskRuntimeCtx,
    ) -> NodeTaskRuntime {
        let handle_fn: HandlerFn<NodeTask, NodeTaskRuntimeCtx> =
            Box::new(|task: NodeTask, ctx: &NodeTaskRuntimeCtx| {
                Box::pin(async {
                    handle_task(task, ctx).await;
                })
            });

        let task_runtime = TaskRuntime::new(
            node_task_queue_clone,
            node_task_min_interval,
            node_task_runtime_ctx,
            handle_fn,
        );

        NodeTaskRuntime { task_runtime }
    }

    pub async fn run(self) {
        self.task_runtime.run().await;
    }
}

async fn handle_task(
    task: NodeTask,
    ctx: &NodeTaskRuntimeCtx,
) -> Result<(), SaksahaNodeError> {
    match task {
        NodeTask::SendHello { her_public_key } => {}
        NodeTask::SendTxHashSyn {
            tx_hashes,
            her_public_key,
        } => {}
        NodeTask::SendTxSyn {
            tx_candidates,
            her_public_key,
        } => {}
        NodeTask::SendBlockHashSyn {
            new_blocks,
            her_public_key,
        } => {}
    };

    Ok(())
}
