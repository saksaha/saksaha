use crate::node::SaksahaNodeError;

use super::{NodeRuntimeCtx, NodeTask};
use sak_task_queue::{HandlerFn, TaskQueue, TaskRuntime};
use std::{pin::Pin, sync::Arc};

pub(in crate::node) fn create_node_task_runtime(
    node_task_queue_clone: Arc<TaskQueue<NodeTask>>,
    node_task_min_interval: Option<u64>,
    node_runtime_ctx: NodeRuntimeCtx,
) -> TaskRuntime<NodeTask, NodeRuntimeCtx> {
    let handle_fn: HandlerFn<NodeTask> = Box::new(|task: NodeTask| {
        Box::pin(async {
            handle_task(task).await;
        })
    });

    let node_task_runtime = TaskRuntime::new(
        node_task_queue_clone,
        node_task_min_interval,
        node_runtime_ctx,
        handle_fn,
    );

    node_task_runtime
}

async fn handle_task(task: NodeTask) -> Result<(), SaksahaNodeError> {
    match task {
        NodeTask::SendHello { her_public_key } => {}
        NodeTask::SendTxSyn {
            tx_candidates,
            her_public_key,
        } => {}
    };

    Ok(())
}
