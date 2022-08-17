use crate::{
    machine::Machine,
    node::{task::NodeTask, SaksahaNodeError},
};
use log::{debug, info, warn};
use sak_p2p_transport::{
    BlockHashSyncMsg, Msg, RecvReceipt, SendReceipt, UpgradedConn,
};
use sak_task_queue::TaskQueue;
use sak_types::{BlockHash, BlockHeight};
use std::sync::Arc;
use tokio::sync::RwLockWriteGuard;

pub(in crate::node) async fn send_block_hash_syn(
    mut conn_lock: RwLockWriteGuard<'_, UpgradedConn>,
    new_blocks: Vec<(BlockHeight, BlockHash)>,
    task_queue: &Arc<TaskQueue<NodeTask>>,
) -> Result<RecvReceipt, SaksahaNodeError> {
    conn_lock
        .send(Msg::BlockHashSyn(BlockHashSyncMsg {
            new_blocks: new_blocks.clone(),
        }))
        .await?;

    let (msg, receipt) = conn_lock.next_msg().await;

    let msg =
        msg.ok_or(format!("block hash syn needs to be followed by ack"))??;

    let block_hash_ack_msg = match msg {
        Msg::BlockHashAck(m) => m,
        _ => {
            return Err(format!(
                "Only block hash ack should arrive at this point"
            )
            .into());
        }
    };

    let new_blocks = block_hash_ack_msg.new_blocks;

    task_queue
        .push_back(NodeTask::SendBlockSyn { new_blocks })
        .await?;

    Ok(receipt)
}

pub(in crate::node) async fn recv_block_hash_syn(
    block_hash_syn_msg: BlockHashSyncMsg,
    machine: &Arc<Machine>,
    mut conn: RwLockWriteGuard<'_, UpgradedConn>,
) -> Result<SendReceipt, SaksahaNodeError> {
    let new_blocks = block_hash_syn_msg.new_blocks;

    let (_, latest_block_hash) = machine
        .blockchain
        .dist_ledger
        .apis
        .get_latest_block_hash()
        .await?
        .ok_or("height does not exist")?;

    debug!(
        "handle block hash syn, latest_block_hash: {}, \
            received_new_blocks: {:?}",
        latest_block_hash, new_blocks,
    );

    let mut blocks_to_req = vec![];
    for (height, block_hash) in new_blocks {
        if block_hash != latest_block_hash {
            blocks_to_req.push((height, block_hash));
        }
    }

    let receipt = conn
        .send(Msg::BlockHashAck(BlockHashSyncMsg {
            new_blocks: blocks_to_req,
        }))
        .await?;

    Ok(receipt)
}
