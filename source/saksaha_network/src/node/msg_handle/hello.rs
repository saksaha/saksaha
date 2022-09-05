use crate::{
    machine::Machine,
    node::{task::NodeTask, SaksahaNodeError},
};
use log::{debug, info, warn};
use sak_p2p_addr::UnknownAddr;
use sak_p2p_peertable::{Peer, PeerTable};
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
) -> Result<RecvReceipt, SaksahaNodeError> {
    let hello_syn_msg = HelloMsg::new(unknown_addrs)?;

    let _receipt = conn_lock.send(Msg::HelloSyn(hello_syn_msg)).await;

    let msg_wrap = conn_lock.next_msg().await?;

    let receipt = msg_wrap.get_receipt();

    let msg = msg_wrap
        .get_maybe_msg()
        .ok_or("hello ack should arrive as reply")??;

    let _hello_ack = match msg {
        Msg::HelloAck(m) => m,
        Msg::Error(m) => {
            return Err(
                format!("Receiver returned error msg, msg: {:?}", m).into()
            )
        }
        _ => {
            return Err(format!(
                "Only hello ack should arrive at this point, msg: {}",
                msg,
            )
            .into())
        }
    };

    // enqueue

    Ok(receipt)
}

pub(in crate::node) async fn recv_hello_syn(
    mut conn: RwLockWriteGuard<'_, UpgradedConn>,
    peer_table: &Arc<PeerTable>,
) -> SendReceipt {
    let unknown_addrs = peer_table.get_peer_addrs().await;

    let hello_ack_msg = HelloMsg::new(unknown_addrs).unwrap();

    let receipt = conn.send(Msg::HelloAck(hello_ack_msg)).await;

    receipt
}
