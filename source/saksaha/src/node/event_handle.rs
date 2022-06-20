use crate::machine::Machine;
use futures::{SinkExt, StreamExt};
use log::{info, warn};
use sak_p2p_trpt::{Msg, TxHashSync, TxSyn, UpgradedConnection};
use std::time::Duration;
use tokio::sync::RwLockWriteGuard;

const RESPONSE_TIMEOUT: u64 = 2000;

pub(super) async fn handle_tx_pool_stat<'a>(
    public_key: &str,
    conn: &'a mut RwLockWriteGuard<'_, UpgradedConnection>,
    machine: &Machine,
    new_tx_hashes: Vec<String>,
) {
    match conn
        .socket
        .send(Msg::TxHashSyn(TxHashSync {
            tx_hashes: new_tx_hashes,
        }))
        .await
    {
        Ok(_) => {
            info!("Sending TxHashSyn, public_key: {}", public_key);
        }
        Err(err) => {
            warn!(
                "Failed to request to synchronize with peer node, err: {}",
                err,
            );
        }
    };

    let resp_timeout =
        tokio::time::sleep(Duration::from_millis(RESPONSE_TIMEOUT));

    let txs = tokio::select! {
        _ = resp_timeout => {
            warn!("Peer did not respond in time, public_key: {}", public_key);
            return;
        },
        resp = conn.socket.next() => {
            match resp {
                Some(maybe_msg) => match maybe_msg {
                    Ok(msg) => match msg {
                        Msg::TxHashAck(h) => {
                            let txs = machine
                                .blockchain
                                .get_ack_txs_from_pool(h.tx_hashes)
                                .await;

                            txs
                        }
                        _ => {
                            warn!("Received an invalid type message");
                            return;
                        }
                    },
                    Err(err) => {
                        warn!("Failed to parse the msg, err: {}", err);
                        return;
                    }
                },
                None => {
                    warn!("Received an invalid data stream");
                    return;
                }
            }
        },
    };

    match conn.socket.send(Msg::TxSyn(TxSyn { txs })).await {
        Ok(_) => {
            info!("Sending TxSyn, public_key: {}", public_key);
        }
        Err(err) => {
            info!("Failed to send requested tx, err: {}", err,);
        }
    }
}
