use crate::{
    machine::Machine,
    node::{task::NodeTask, SaksahaNodeError},
};
use log::{debug, info, warn};
use sak_p2p_addr::UnknownAddr;
use sak_p2p_discovery::Discovery;
use sak_p2p_peertable::{Peer, PeerTable};
use sak_p2p_transport::{HelloMsg, Msg, TxHashSyncMsg, UpgradedConn};
use sak_task_queue::TaskQueue;
use sak_types::TxHash;
use std::sync::Arc;
use tokio::sync::RwLockWriteGuard;

pub(in crate::node) async fn send_hello_syn(
    mut conn_lock: RwLockWriteGuard<'_, UpgradedConn>,
    discovery: &Arc<Discovery>,
    unknown_addrs: Vec<UnknownAddr>,
    task_queue: &Arc<TaskQueue<NodeTask>>,
) -> Result<(), SaksahaNodeError> {
    let hello_syn_msg = HelloMsg::new(unknown_addrs)?;

    let _receipt = conn_lock.send(Msg::HelloSyn(hello_syn_msg)).await;

    Ok(())
}

pub(in crate::node) async fn recv_hello_ack(
    hello_ack: HelloMsg,
    peer_table: &Arc<PeerTable>,
    discovery: &Arc<Discovery>,
    mut conn: RwLockWriteGuard<'_, UpgradedConn>,
) -> Result<(), SaksahaNodeError> {
    let HelloMsg { unknown_addrs } = hello_ack;

    for unknown_addr in unknown_addrs {
        discovery.enqueue_who_are_you(&unknown_addr).await;
    }

    Ok(())
}

pub(in crate::node) async fn recv_hello_syn(
    hello_msg: HelloMsg,
    peer_table: &Arc<PeerTable>,
    discovery: &Arc<Discovery>,
    mut conn: RwLockWriteGuard<'_, UpgradedConn>,
) -> Result<(), SaksahaNodeError> {
    let HelloMsg { unknown_addrs } = hello_msg;

    for unknown_addr in unknown_addrs {
        discovery.enqueue_who_are_you(&unknown_addr).await;
    }

    let unknown_addrs = peer_table.get_peer_addrs().await;

    let hello_ack_msg = HelloMsg::new(unknown_addrs).unwrap();

    conn.send(Msg::HelloAck(hello_ack_msg)).await;

    Ok(())
}
