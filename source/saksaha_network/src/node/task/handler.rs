use super::NodeTask;
use crate::{
    machine::Machine,
    node::{msg_handle, SaksahaNodeError},
};
use log::{debug, error, warn};
use sak_p2p_discovery::Discovery;
use sak_p2p_transport::UpgradedConn;
use sak_task_queue::TaskQueue;
use std::sync::Arc;
use tokio::sync::RwLockWriteGuard;

pub(in crate::node) async fn handle_task<'a>(
    task: NodeTask,
    task_queue: &Arc<TaskQueue<NodeTask>>,
    conn_lock: RwLockWriteGuard<'a, UpgradedConn>,
    machine: &Arc<Machine>,
    discovery: &Arc<Discovery>,
) -> Result<(), SaksahaNodeError> {
    // let task_type = task.to_string();

    println!("handling task: {}", &task);

    let res = match task {
        NodeTask::SendHelloSyn { unknown_addrs } => {
            msg_handle::send_hello_syn(
                conn_lock,
                discovery,
                unknown_addrs,
                task_queue,
            )
            .await?;
        }
        NodeTask::SendTxHashSyn { tx_hashes } => {
            msg_handle::send_tx_hash_syn(conn_lock, tx_hashes, task_queue)
                .await?;
        }
        NodeTask::SendTxSyn { tx_hashes } => {
            msg_handle::send_tx_syn(conn_lock, tx_hashes, &machine).await?;
        }
        NodeTask::SendBlockHashSyn { new_blocks } => {
            msg_handle::send_block_hash_syn(conn_lock, new_blocks, task_queue)
                .await?;
        }
        NodeTask::SendBlockSyn { new_blocks } => {
            msg_handle::send_block_syn(conn_lock, new_blocks, &machine).await?;
        }
    };

    // if let Err(err) = res {
    //     warn!("Task handle failed, task: {}, err: {}", task_type, err);
    // }

    Ok(())
}
