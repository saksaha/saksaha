mod block;
mod block_hash;
mod hello;
mod tx;
mod tx_hash;

use super::task::NodeTask;
use crate::{machine::Machine, SaksahaError};
pub(in crate::node) use block::*;
pub(in crate::node) use block_hash::*;
pub(in crate::node) use hello::*;
use sak_logger::{debug, info, warn};
use sak_p2p_discovery::Discovery;
use sak_p2p_peertable::{Peer, PeerTable};
use sak_p2p_transport::{Msg, TxHashSyncMsg, TxSynMsg, UpgradedConn, UpgradedP2PCodec};
use sak_task_queue::TaskQueue;
use std::sync::Arc;
use tokio::{net::TcpStream, sync::RwLockWriteGuard};
pub(in crate::node) use tx::*;
pub(in crate::node) use tx_hash::*;

pub(in crate::node) async fn handle_msg<'a>(
    msg: Msg,
    machine: &Arc<Machine>,
    conn_lock: RwLockWriteGuard<'_, UpgradedConn>,
    task_queue: &Arc<TaskQueue<NodeTask>>,
    peer_table: &Arc<PeerTable>,
    discovery: &Arc<Discovery>,
) -> Result<(), SaksahaError> {
    match msg {
        Msg::HelloSyn(hello_msg) => {
            hello::recv_hello_syn(hello_msg, peer_table, discovery, conn_lock).await?;
        }
        Msg::HelloAck(hello_msg) => {
            hello::recv_hello_ack(hello_msg, peer_table, discovery, conn_lock).await?;
        }
        Msg::TxHashSyn(tx_hash_sync) => {
            tx_hash::recv_tx_hash_syn(tx_hash_sync, machine, conn_lock).await?;
        }
        Msg::TxHashAck(tx_hash_sync) => {
            tx_hash::recv_tx_hash_ack(tx_hash_sync, task_queue).await?;
        }
        Msg::TxSyn(tx_syn) => {
            tx::recv_tx_syn(tx_syn, machine, conn_lock).await?;
        }
        Msg::TxAck(tx_ack) => {
            tx::recv_tx_ack(tx_ack, machine, conn_lock).await?;
        }
        Msg::BlockHashSyn(block_hash_syn) => {
            block_hash::recv_block_hash_syn(block_hash_syn, machine, conn_lock).await?;
        }
        Msg::BlockHashAck(block_hash_ack) => {
            block_hash::recv_block_hash_ack(block_hash_ack, task_queue).await?;
        }
        Msg::BlockSyn(block_syn_msg) => {
            block::recv_block_syn(block_syn_msg, machine, conn_lock).await?;
        }
        Msg::BlockAck(block_ack_msg) => {
            block::recv_block_ack(block_ack_msg, machine).await?;
        }
        _ => {
            return Err(format!("Msg not valid at this stage, discarding, msg: {:?}", msg).into());
        }
    };

    Ok(())
}
