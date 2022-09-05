use crate::{
    machine::Machine,
    node::{task::NodeTask, SaksahaNodeError},
};
use log::{debug, info, warn};
use sak_p2p_addr::UnknownAddr;
use sak_p2p_peertable::Peer;
use sak_p2p_transport::{
    HelloMsg, Msg, RecvReceipt, SendReceipt, TxHashSyncMsg, UpgradedConn,
};
use sak_task_queue::TaskQueue;
use sak_types::TxHash;
use std::sync::Arc;
use tokio::sync::RwLockWriteGuard;

pub(in crate::node) async fn send_hello_syn(
    mut conn_lock: RwLockWriteGuard<'_, UpgradedConn>,
    unknown_addrs: Vec<UnknownAddr>,
    task_queue: &Arc<TaskQueue<NodeTask>>,
) -> Result<RecvReceipt, SaksahaNodeError> {
    let hello_syn_msg = HelloMsg::new()?;

    let _receipt = conn_lock.send(Msg::HelloSyn(hello_syn_msg)).await;

    let msg_wrap = conn_lock.next_msg().await?;

    let receipt = msg_wrap.get_receipt();

    let msg = msg_wrap
        .get_maybe_msg()
        .ok_or("tx hash ack should arrive as reply")??;

    let tx_hash_ack = match msg {
        Msg::TxHashAck(m) => m,
        Msg::Error(m) => {
            return Err(
                format!("Receiver returned error msg, msg: {:?}", m).into()
            )
        }
        _ => {
            return Err(format!(
                "Only tx hash ack should arrive at this point, msg: {}",
                msg,
            )
            .into())
        }
    };

    task_queue
        .push_back(NodeTask::SendHelloSyn { unknown_addrs })
        .await?;

    Ok(receipt)
}

pub(in crate::node) async fn recv_hello_syn(
    tx_hash_syn_msg: TxHashSyncMsg,
    machine: &Arc<Machine>,
    mut conn: RwLockWriteGuard<'_, UpgradedConn>,
    task_queue: &Arc<TaskQueue<NodeTask>>,
    peer: &Arc<Peer>,
) -> SendReceipt {
    let txs_to_request = machine
        .blockchain
        .dist_ledger
        .apis
        .get_tx_pool_diff(tx_hash_syn_msg.tx_hashes)
        .await;

    //
    let hello_ack_msg = HelloMsg::new();

    let receipt = conn
        .send(Msg::TxHashAck(TxHashSyncMsg {
            tx_hashes: txs_to_request,
        }))
        .await;

    receipt
}
