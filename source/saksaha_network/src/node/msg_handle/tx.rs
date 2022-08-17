use crate::{machine::Machine, node::SaksahaNodeError};
use log::{debug, info, warn};
use sak_p2p_transport::{
    Msg, RecvReceipt, SendReceipt, TxAckMsg, TxSynMsg, UpgradedConn,
};
use sak_types::TxHash;
use std::sync::Arc;
use tokio::sync::RwLockWriteGuard;

pub(in crate::node) async fn send_tx_syn<'a>(
    mut conn_lock: RwLockWriteGuard<'a, UpgradedConn>,
    tx_hashes: Vec<TxHash>,
    machine: &Arc<Machine>,
) -> Result<RecvReceipt, SaksahaNodeError> {
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
