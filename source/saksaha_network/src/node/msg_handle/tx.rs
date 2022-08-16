use crate::{
    machine::Machine,
    node::{task::NodeTask, SaksahaNodeError},
    SaksahaError,
};
use futures::{stream::SplitSink, SinkExt};
use log::{debug, info, warn};
use sak_p2p_peertable::{Peer, PeerTable};
use sak_p2p_transport::{
    BlockHashSyncMsg, BlockSynMsg, Msg, MsgType, RecvReceipt, SendReceipt,
    TxAckMsg, TxHashSyncMsg, TxSynMsg, UpgradedConn, UpgradedP2PCodec,
};
use sak_task_queue::TaskQueue;
use sak_types::{TxCandidate, TxHash};
use std::sync::Arc;
use tokio::{net::TcpStream, sync::RwLockWriteGuard};

pub(in crate::node) async fn send_tx_syn<'a>(
    mut conn_lock: RwLockWriteGuard<'a, UpgradedConn>,
    tx_hashes: Vec<TxHash>,
    machine: &Arc<Machine>,
) -> Result<RecvReceipt, SaksahaNodeError> {
    println!("send tx syn");

    let tx_candidates = machine
        .blockchain
        .dist_ledger
        .apis
        .get_txs_from_pool(tx_hashes)
        .await;

    let tx_syn_msg = Msg::TxSyn(TxSynMsg { tx_candidates });

    conn_lock.send(tx_syn_msg).await?;

    let (msg, receipt) = conn_lock.next_msg().await;

    let msg =
        msg.ok_or(format!("tx syn needs to be followed by tx syn ack"))??;

    let _tx_ack = match msg {
        Msg::TxAck(m) => m,
        _ => {
            return Err(
                format!("Only tx ack should arrive at this point").into()
            );
        }
    };

    Ok(receipt)
}

pub(in crate::node) async fn recv_tx_syn(
    tx_syn: TxSynMsg,
    machine: &Machine,
    mut conn: RwLockWriteGuard<'_, UpgradedConn>,
) -> Result<SendReceipt, SaksahaNodeError> {
    println!("recv tx syn");

    machine
        .blockchain
        .dist_ledger
        .apis
        .insert_into_pool(tx_syn.tx_candidates)
        .await;

    let tx_ack_msg = Msg::TxAck(TxAckMsg {});

    let receipt = conn.send(tx_ack_msg).await?;

    Ok(receipt)
}

// pub(super) async fn handle_tx_hash_syn<'a>(
//     tx_hash_sync: TxHashSyncMsg,
//     machine: &Machine,
//     conn: &'a mut RwLockWriteGuard<'_, UpgradedConn>,
//     task_queue: &Arc<TaskQueue<NodeTask>>,
//     peer: &Arc<Peer>,
// ) -> Result<(), SaksahaNodeError> {
//     let txs_to_request = machine
//         .blockchain
//         .dist_ledger
//         .apis
//         .get_tx_pool_diff(tx_hash_sync.tx_hashes)
//         .await;

//     match conn
//         .send(Msg::TxHashAck(TxHashSyncMsg {
//             tx_hashes: txs_to_request,
//         }))
//         .await
//     {
//         Ok(_) => {}
//         Err(err) => {
//             warn!("Failed to handle TxHashSyn msg, err: {}", err,);
//         }
//     };

//     let msg = conn
//         .next_msg()
//         .await
//         .ok_or(format!("tx hash ack is empty"))??;

//     let tx_hash_ack = match msg {
//         Msg::TxHashAck(m) => m,
//         _ => {
//             return Err(format!(
//                 "Only tx hash msg is expected, msg: {:?}",
//                 msg
//             )
//             .into());
//         }
//     };

//     let tx_candidates = machine
//         .blockchain
//         .dist_ledger
//         .apis
//         .get_txs_from_pool(tx_hash_ack.tx_hashes)
//         .await;

//     task_queue
//         .push_back(NodeTask::SendTxSyn { tx_candidates })
//         .await?;

//     // if !tx_candidates.is_empty() {
//     //     match conn.send(Msg::TxSyn(TxSynMsg { tx_candidates })).await {
//     //         Ok(_) => {
//     //             // info!("Sending TxSyn, public_key: {}", public_key);
//     //         }
//     //         Err(err) => {
//     //             info!("Failed to handle TxHashAck, err: {}", err,);
//     //         }
//     //     }
//     // }

//     Ok(())
// }

// pub(super) async fn handle_tx_hash_ack<'a>(
//     tx_hash_ack: TxHashSynMsg,
//     machine: &Machine,
//     conn: &'a mut RwLockWriteGuard<'_, UpgradedConn>,
// ) {
//     let tx_candidates = machine
//         .blockchain
//         .dist_ledger
//         .apis
//         .get_txs_from_pool(tx_hash_ack.tx_hashes)
//         .await;

//     if !tx_candidates.is_empty() {
//         match conn.send(Msg::TxSyn(TxSynMsg { tx_candidates })).await {
//             Ok(_) => {
//                 // info!("Sending TxSyn, public_key: {}", public_key);
//             }
//             Err(err) => {
//                 info!("Failed to handle TxHashAck, err: {}", err,);
//             }
//         }
//     }
// }

pub(super) async fn handle_tx_syn(
    tx_syn: TxSynMsg,
    machine: &Machine,
) -> Result<(), SaksahaError> {
    machine
        .blockchain
        .dist_ledger
        .apis
        .insert_into_pool(tx_syn.tx_candidates)
        .await;

    Ok(())
}
