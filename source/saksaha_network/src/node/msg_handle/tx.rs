use crate::node::{task::NodeTask, SaksahaNodeError};
use sak_logger::{debug, info, warn};
use sak_machine::SakMachine;
use sak_p2p_transport::{ErrorMsg, Msg, TxAckMsg, TxSynMsg, UpgradedConn};
use sak_task_queue::TaskQueue;
use sak_types::TxHash;
use std::sync::Arc;
use tokio::sync::RwLockWriteGuard;

pub(in crate::node) async fn send_tx_syn<'a>(
    mut conn_lock: RwLockWriteGuard<'a, UpgradedConn>,
    tx_hashes: Vec<TxHash>,
    machine: &Arc<SakMachine>,
) -> Result<(), SaksahaNodeError> {
    let tx_candidates = machine
        .ledger
        // .dist_ledger
        .get_txs_from_pool(tx_hashes)
        .await;

    let tx_syn_msg = Msg::TxSyn(TxSynMsg { tx_candidates });

    conn_lock.send(tx_syn_msg).await;

    Ok(())
}

pub(in crate::node) async fn recv_tx_ack(
    tx_syn: TxAckMsg,
    machine: &SakMachine,
    mut conn_lock: RwLockWriteGuard<'_, UpgradedConn>,
) -> Result<(), SaksahaNodeError> {
    Ok(())
}

pub(in crate::node) async fn recv_tx_syn(
    tx_syn: TxSynMsg,
    machine: &SakMachine,
    mut conn_lock: RwLockWriteGuard<'_, UpgradedConn>,
) -> Result<(), SaksahaNodeError> {
    machine
        .ledger
        // .dist_ledger
        .insert_into_pool(tx_syn.tx_candidates)
        .await;

    let tx_ack_msg = Msg::TxAck(TxAckMsg {});

    conn_lock.send(tx_ack_msg).await;

    Ok(())
}
