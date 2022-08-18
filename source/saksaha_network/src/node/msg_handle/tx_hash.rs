use crate::{
    machine::Machine,
    node::{task::NodeTask, SaksahaNodeError},
};
use log::{debug, info, warn};
use sak_p2p_peertable::Peer;
use sak_p2p_transport::{
    Msg, RecvReceipt, SendReceipt, TxHashSyncMsg, UpgradedConn,
};
use sak_task_queue::TaskQueue;
use sak_types::TxHash;
use std::sync::Arc;
use tokio::sync::RwLockWriteGuard;

pub(in crate::node) async fn send_tx_hash_syn(
    mut conn_lock: RwLockWriteGuard<'_, UpgradedConn>,
    tx_hashes: Vec<TxHash>,
    task_queue: &Arc<TaskQueue<NodeTask>>,
) -> Result<RecvReceipt, SaksahaNodeError> {
    let _receipt = conn_lock
        .send(Msg::TxHashSyn(TxHashSyncMsg { tx_hashes }))
        .await;

    let msg_wrap = conn_lock.next_msg().await?;

    let receipt = msg_wrap.get_receipt();

    let msg = msg_wrap
        .get_maybe_msg()
        .ok_or("tx hash ack should arrive as reply")??;

    let tx_hash_ack = match msg {
        Msg::TxHashAck(m) => m,
        _ => {
            return Err(format!(
                "Only tx hash ack should arrive at this point, msg: {}",
                msg,
            )
            .into())
        }
    };

    task_queue
        .push_back(NodeTask::SendTxSyn {
            tx_hashes: tx_hash_ack.tx_hashes,
        })
        .await?;

    Ok(receipt)
}

pub(in crate::node) async fn recv_tx_hash_syn(
    tx_hash_syn_msg: TxHashSyncMsg,
    machine: &Arc<Machine>,
    mut conn: RwLockWriteGuard<'_, UpgradedConn>,
    task_queue: &Arc<TaskQueue<NodeTask>>,
    peer: &Arc<Peer>,
) -> Result<SendReceipt, SaksahaNodeError> {
    let txs_to_request = machine
        .blockchain
        .dist_ledger
        .apis
        .get_tx_pool_diff(tx_hash_syn_msg.tx_hashes)
        .await;

    let receipt = conn
        .send(Msg::TxHashAck(TxHashSyncMsg {
            tx_hashes: txs_to_request,
        }))
        .await;

    Ok(receipt)
}
