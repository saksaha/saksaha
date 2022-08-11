mod block;
mod block_hash;
mod tx;
mod tx_hash;

pub(in crate::node) use block::*;
pub(in crate::node) use block_hash::*;
pub(in crate::node) use tx::*;
pub(in crate::node) use tx_hash::*;

// use crate::{machine::Machine, SaksahaError};
// use futures::{stream::SplitSink, SinkExt};
// use log::{debug, info, warn};
// use sak_p2p_peertable::Peer;
// use sak_p2p_transport::{
//     BlockHashSynMsg, BlockSynMsg, Msg, TxHashSynMsg, TxSynMsg, UpgradedConn,
//     UpgradedP2PCodec,
// };
// use sak_task_queue::TaskQueue;
// use std::sync::Arc;
// use tokio::{net::TcpStream, sync::RwLockWriteGuard};
// use tokio_util::codec::Framed;

// use super::task::NodeTask;

// pub(in crate::node) async fn handle_msg<'a>(
//     msg: Msg,
//     machine: &Machine,
//     conn: &'a mut RwLockWriteGuard<'_, UpgradedConn>,
//     task_queue: &Arc<TaskQueue<NodeTask>>,
//     peer: &Arc<Peer>,
// ) -> Result<(), SaksahaError> {
//     match msg {
//         Msg::TxHashSyn(tx_hash_syn) => {
//             tx::handle_tx_hash_syn(
//                 tx_hash_syn,
//                 machine,
//                 conn,
//                 task_queue,
//                 peer,
//             )
//             .await;
//         }
//         // Msg::TxHashAck(tx_hash_ack) => {
//         //     handle_tx_hash_ack(public_key, tx_hash_ack, machine, conn).await;
//         // }
//         Msg::TxSyn(tx_syn) => {
//             tx::handle_tx_syn(tx_syn, machine).await?;
//         }
//         Msg::BlockHashSyn(block_hash_syn_msg) => {
//             block::handle_block_hash_syn(block_hash_syn_msg, machine, conn)
//                 .await?;
//         }
//         Msg::BlockSyn(block_syn_msg) => {
//             block::handle_block_syn(block_syn_msg, machine).await?;
//         }
//         // Msg::BlockHashAck(block_hash_syn_msg) => {
//         //     let _ =
//         //         handle_block_hash_ack(block_hash_syn_msg, machine, conn).await;
//         // }
//         _ => {
//             warn!("Msg not valid at this stage, discarding, msg: {:?}", msg);
//         }
//     };

//     Ok(())
// }
