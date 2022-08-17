mod block;
mod block_hash;
mod tx;
mod tx_hash;

use super::task::NodeTask;
use crate::{machine::Machine, SaksahaError};
pub(in crate::node) use block::*;
pub(in crate::node) use block_hash::*;
use futures::{stream::SplitSink, SinkExt};
use log::{debug, info, warn};
use sak_p2p_peertable::Peer;
use sak_p2p_transport::{
    BlockHashSyncMsg, BlockSynMsg, Msg, SendReceipt, TxHashSyncMsg, TxSynMsg,
    UpgradedConn, UpgradedP2PCodec,
};
use sak_task_queue::TaskQueue;
use std::sync::Arc;
use tokio::{net::TcpStream, sync::RwLockWriteGuard};
pub(in crate::node) use tx::*;
pub(in crate::node) use tx_hash::*;

pub(in crate::node) async fn handle_msg<'a>(
    msg: Msg,
    machine: &Arc<Machine>,
    conn: RwLockWriteGuard<'_, UpgradedConn>,
    task_queue: &Arc<TaskQueue<NodeTask>>,
    peer: &Arc<Peer>,
) -> Result<(), SaksahaError> {
    let _: SendReceipt = match msg {
        Msg::TxHashSyn(tx_hash_syn) => {
            tx_hash::recv_tx_hash_syn(
                tx_hash_syn,
                machine,
                conn,
                task_queue,
                peer,
            )
            .await?
        }
        Msg::TxSyn(tx_syn) => tx::recv_tx_syn(tx_syn, machine, conn).await?,
        Msg::BlockHashSyn(block_hash_syn) => {
            block_hash::recv_block_hash_syn(block_hash_syn, machine, conn)
                .await?
        }
        Msg::BlockSyn(block_syn_msg) => {
            block::recv_block_syn(block_syn_msg, machine, conn).await?
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
