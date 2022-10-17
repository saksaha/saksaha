use crate::node::{task::NodeTask, SaksahaNodeError};
use sak_logger::{debug, info, warn};
use sak_machine::SakMachine;
use sak_p2p_peertable::Peer;
use sak_p2p_transport::{Msg, TxHashSyncMsg, UpgradedConn};
use sak_task_queue::TaskQueue;
use sak_types::TxHash;
use std::sync::Arc;
use tokio::sync::RwLockWriteGuard;

pub(in crate::node) async fn send_tx_hash_syn(
    mut conn_lock: RwLockWriteGuard<'_, UpgradedConn>,
    tx_hashes: Vec<TxHash>,
) -> Result<(), SaksahaNodeError> {
    let _receipt = conn_lock
        .send(Msg::TxHashSyn(TxHashSyncMsg { tx_hashes }))
        .await;

    Ok(())
}

pub(in crate::node) async fn recv_tx_hash_ack(
    tx_hash_sync_msg: TxHashSyncMsg,
    task_queue: &Arc<TaskQueue<NodeTask>>,
) -> Result<(), SaksahaNodeError> {
    task_queue
        .push_back(NodeTask::SendTxSyn {
            tx_hashes: tx_hash_sync_msg.tx_hashes,
        })
        .await?;

    Ok(())
}

pub(in crate::node) async fn recv_tx_hash_syn(
    tx_hash_syn_msg: TxHashSyncMsg,
    machine: &Arc<SakMachine>,
    mut conn: RwLockWriteGuard<'_, UpgradedConn>,
) -> Result<(), SaksahaNodeError> {
    let txs_to_request = machine
        .ledger
        // .dist_ledger
        .get_tx_pool_diff(tx_hash_syn_msg.tx_hashes)
        .await;

    conn.send(Msg::TxHashAck(TxHashSyncMsg {
        tx_hashes: txs_to_request,
    }))
    .await;

    Ok(())
}
