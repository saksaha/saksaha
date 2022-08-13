use crate::{
    machine::Machine,
    node::{task::NodeTask, SaksahaNodeError},
    SaksahaError,
};
use futures::{stream::SplitSink, SinkExt};
use log::{debug, info, warn};
use sak_p2p_peertable::Peer;
use sak_p2p_transport::{
    BlockHashSynMsg, BlockSynMsg, Msg, SendReceipt, TxHashSyncMsg, TxSynMsg,
    UpgradedConn, UpgradedP2PCodec,
};
use sak_task_queue::TaskQueue;
use sak_types::TxHash;
use std::sync::Arc;
use tokio::{net::TcpStream, sync::RwLockWriteGuard};

pub(in crate::node) async fn send_tx_hash_syn(
    peer: &Arc<Peer>,
    tx_hashes: Vec<TxHash>,
) -> Result<(), SaksahaNodeError> {
    let mut conn_lock = peer.get_transport().conn.write().await;

    let receipt = conn_lock
        .send(Msg::TxHashSyn(TxHashSyncMsg { tx_hashes }))
        .await?;

    let msg = conn_lock
        .next_msg()
        .await
        .ok_or("tx hash ack should arrive as reply")??;

    match msg {
        Msg::TxHashAck(m) => {
            println!("tx hash ack received");
        }
        _ => {
            return Err(format!(
                "Only tx hash ack should arrive at this point, msg: {}",
                msg,
            )
            .into());
        }
    }

    Ok(())
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
        .await?;

    Ok(receipt)
}

pub(super) async fn handle_tx_hash_syn<'a>(
    tx_hash_syn_msg: TxHashSyncMsg,
    machine: &Machine,
    conn: &'a mut RwLockWriteGuard<'_, UpgradedConn>,
    task_queue: &Arc<TaskQueue<NodeTask>>,
    peer: &Arc<Peer>,
) -> Result<(), SaksahaNodeError> {
    let txs_to_request = machine
        .blockchain
        .dist_ledger
        .apis
        .get_tx_pool_diff(tx_hash_syn_msg.tx_hashes)
        .await;

    match conn
        .send(Msg::TxHashAck(TxHashSyncMsg {
            tx_hashes: txs_to_request,
        }))
        .await
    {
        Ok(_) => {}
        Err(err) => {
            warn!("Failed to handle TxHashSyn msg, err: {}", err,);
        }
    };

    let msg = conn
        .next_msg()
        .await
        .ok_or(format!("tx hash ack is empty"))??;

    let tx_hash_ack = match msg {
        Msg::TxHashAck(m) => m,
        _ => {
            return Err(format!(
                "Only tx hash msg is expected, msg: {:?}",
                msg
            )
            .into());
        }
    };

    let tx_candidates = machine
        .blockchain
        .dist_ledger
        .apis
        .get_txs_from_pool(tx_hash_ack.tx_hashes)
        .await;

    task_queue
        .push_back(NodeTask::SendTxSyn { tx_candidates })
        .await?;

    // if !tx_candidates.is_empty() {
    //     match conn.send(Msg::TxSyn(TxSynMsg { tx_candidates })).await {
    //         Ok(_) => {
    //             // info!("Sending TxSyn, public_key: {}", public_key);
    //         }
    //         Err(err) => {
    //             info!("Failed to handle TxHashAck, err: {}", err,);
    //         }
    //     }
    // }

    Ok(())
}

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
