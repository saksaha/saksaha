use futures::{SinkExt, StreamExt};
use log::{info, warn};
use sak_p2p_trpt::{Connection, Msg, TxHashSyn, TxSyn};
use tokio::sync::RwLockWriteGuard;

use crate::machine::Machine;

pub(super) async fn handle_tx_pool_stat<'a>(
    conn: &'a mut RwLockWriteGuard<'_, Connection>,
    machine: &Machine,
    new_tx_hashes: Vec<String>,
) {
    match conn
        .socket
        .send(Msg::TxHashSyn(TxHashSyn {
            tx_hashes: new_tx_hashes,
        }))
        .await
    {
        Ok(_) => {
            info!(
                "Incoming tx successfully requested to sync with \
                the peer node, send msg type: TxHashSyn"
            );

            match conn.socket.next().await {
                Some(maybe_msg) => match maybe_msg {
                    Ok(msg) => match msg {
                        Msg::TxHashAck(h) => {
                            let txs = machine
                                .blockchain
                                .get_ack_txs_from_pool(h.tx_hashes)
                                .await;

                            match conn
                                .socket
                                .send(Msg::TxSyn(TxSyn { txs }))
                                .await
                            {
                                Ok(_) => {
                                    info!(
                                        "requested transactions are successfully \
                                        transmitted to the peer node, send msg \
                                        type: TxHSyn"
                                    );
                                }
                                Err(err) => {
                                    info!(
                                        "Failed to send requested tx, err: {}",
                                        err,
                                    );
                                }
                            }
                        }
                        _ => {
                            warn!("Received an invalid type message");
                        }
                    },
                    Err(err) => {
                        warn!("Failed to parse the msg, err: {}", err);
                    }
                },
                None => {
                    warn!("Received an invalid data stream");
                }
            };
        }
        Err(err) => {
            warn!(
                "Failed to request to synchronize with peer node, err: {}",
                err,
            );
        }
    };
}
