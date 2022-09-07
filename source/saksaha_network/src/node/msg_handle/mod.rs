mod block;
mod block_hash;
mod hello;
mod tx;
mod tx_hash;

use super::task::NodeTask;
use crate::{machine::Machine, SaksahaError};
pub(in crate::node) use block::*;
pub(in crate::node) use block_hash::*;
use futures::{stream::SplitSink, SinkExt};
pub(in crate::node) use hello::*;
use log::{debug, info, warn};
use sak_p2p_discovery::Discovery;
use sak_p2p_peertable::{Peer, PeerTable};
use sak_p2p_transport::{
    BlockHashSyncMsg, BlockSynMsg, ErrorMsg, Msg, SendReceipt, TxHashSyncMsg,
    TxSynMsg, UpgradedConn, UpgradedP2PCodec,
};
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
    peer: &Arc<Peer>,
    peer_table: &Arc<PeerTable>,
    discovery: &Arc<Discovery>,
) -> Result<(), SaksahaError> {
    println!("\n 22 handle_msg(): msg: {}", msg);

    match msg {
        Msg::HelloSyn(hello_msg) => {
            hello::recv_hello_syn(
                hello_msg, peer_table, discovery, task_queue, conn_lock,
            )
            .await
        }
        Msg::TxHashSyn(tx_hash_syn) => {
            tx_hash::recv_tx_hash_syn(
                tx_hash_syn,
                machine,
                conn_lock,
                task_queue,
                peer,
            )
            .await
        }
        Msg::TxSyn(tx_syn) => tx::recv_tx_syn(tx_syn, machine, conn_lock).await,
        Msg::BlockHashSyn(block_hash_syn) => {
            block_hash::recv_block_hash_syn(block_hash_syn, machine, conn_lock)
                .await
        }
        Msg::BlockSyn(block_syn_msg) => {
            block::recv_block_syn(block_syn_msg, machine, conn_lock).await
        }
        _ => {
            return Err(format!(
                "Msg not valid at this stage, discarding, msg: {:?}",
                msg
            )
            .into());
        }
    };

    Ok(())
}
