use crate::{machine::Machine, node::SaksahaNodeError};
use log::{debug, info, warn};
use sak_p2p_transport::{
    ErrorMsg, Msg, RecvReceipt, SendReceipt, TxAckMsg, TxSynMsg, UpgradedConn,
};
use sak_types::TxHash;
use std::sync::Arc;
use tokio::sync::RwLockWriteGuard;

pub(in crate::node) async fn send_tx_syn<'a>(
    mut conn_lock: RwLockWriteGuard<'a, UpgradedConn>,
    tx_hashes: Vec<TxHash>,
    machine: &Arc<Machine>,
) -> Result<(), SaksahaNodeError> {
    let tx_candidates = machine
        .blockchain
        .dist_ledger
        .apis
        .get_txs_from_pool(tx_hashes)
        .await;

    let tx_syn_msg = Msg::TxSyn(TxSynMsg { tx_candidates });

    conn_lock.send(tx_syn_msg).await;

    println!("333333333");

    // let msg_wrap = conn_lock.next_msg().await?;

    // let receipt = msg_wrap.get_receipt();

    // let msg = msg_wrap
    //     .get_maybe_msg()
    //     .ok_or(format!("tx syn needs to be followed by tx syn ack"))??;

    // let _tx_ack = match msg {
    //     Msg::TxAck(m) => m,
    //     Msg::Error(m) => {
    //         return Err(
    //             format!("Receiver returned error msg, msg: {:?}", m).into()
    //         )
    //     }
    //     _ => {
    //         return Err(format!(
    //             "Only tx ack should arrive at this point, msg: {}",
    //             msg
    //         )
    //         .into());
    //     }
    // };

    Ok(())
}

pub(in crate::node) async fn recv_tx_syn(
    tx_syn: TxSynMsg,
    machine: &Machine,
    mut conn_lock: RwLockWriteGuard<'_, UpgradedConn>,
) -> SendReceipt {
    println!("444444, tx_syn: {:?}", tx_syn);

    let wrapped = || async {
        machine
            .blockchain
            .dist_ledger
            .apis
            .insert_into_pool(tx_syn.tx_candidates)
            .await;

        let tx_ack_msg = Msg::TxAck(TxAckMsg {});

        let receipt = conn_lock.send(tx_ack_msg).await;

        Ok::<_, SaksahaNodeError>(receipt)
    };

    let receipt = match wrapped().await {
        Ok(r) => r,
        Err(err) => {
            conn_lock
                .send(Msg::Error(ErrorMsg {
                    error: err.to_string(),
                }))
                .await
        }
    };

    receipt
}
