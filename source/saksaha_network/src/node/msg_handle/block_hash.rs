use crate::{
    machine::Machine,
    node::{task::NodeTask, SaksahaNodeError},
};
use sak_logger::{debug, info, warn};
use sak_machine::SakMachine;
use sak_p2p_transport::{BlockHashSyncMsg, ErrorMsg, Msg, UpgradedConn};
use sak_task_queue::TaskQueue;
use sak_types::{BlockHash, BlockHeight};
use std::sync::Arc;
use tokio::sync::RwLockWriteGuard;

pub(in crate::node) async fn send_block_hash_syn(
    mut conn_lock: RwLockWriteGuard<'_, UpgradedConn>,
    new_blocks: Vec<(BlockHeight, BlockHash)>,
) -> Result<(), SaksahaNodeError> {
    conn_lock
        .send(Msg::BlockHashSyn(BlockHashSyncMsg {
            new_blocks: new_blocks.clone(),
        }))
        .await;

    Ok(())
}

pub(in crate::node) async fn recv_block_hash_ack(
    block_hash_ack_msg: BlockHashSyncMsg,
    task_queue: &Arc<TaskQueue<NodeTask>>,
) -> Result<(), SaksahaNodeError> {
    let new_blocks = block_hash_ack_msg.new_blocks;

    task_queue
        .push_back(NodeTask::SendBlockSyn { new_blocks })
        .await?;

    Ok(())
}

pub(in crate::node) async fn recv_block_hash_syn(
    block_hash_syn_msg: BlockHashSyncMsg,
    machine: &Arc<SakMachine>,
    mut conn_lock: RwLockWriteGuard<'_, UpgradedConn>,
) -> Result<(), SaksahaNodeError> {
    let new_blocks = block_hash_syn_msg.new_blocks;

    let (_, latest_block_hash) = machine
        .ledger
        // .dist_ledger
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
        if machine
            .ledger
            // .dist_ledger
            .get_block(&block_hash)?
            .is_none()
        {
            blocks_to_req.push((height, block_hash));
        }
    }

    conn_lock
        .send(Msg::BlockHashAck(BlockHashSyncMsg {
            new_blocks: blocks_to_req,
        }))
        .await;

    Ok(())
}
