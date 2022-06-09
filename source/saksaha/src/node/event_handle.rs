use futures::{SinkExt, StreamExt};
use log::{debug, warn};
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
            debug!(
                "Incoming tx successfully requested to sync with \
                the peer node"
            );
        }
        Err(err) => {
            debug!(
                "Failed to request to synchronize with peer node, \
                err: {}",
                err,
            );
        }
    };

    match conn.socket.next().await {
        Some(maybe_msg) => match maybe_msg {
            Ok(msg) => match msg {
                Msg::TxHashAck(h) => {
                    let txs = machine
                        .blockchain
                        .get_ack_txs_from_pool(h.tx_hashes)
                        .await;

                    match conn.socket.send(Msg::TxSyn(TxSyn { txs })).await {
                        Ok(_) => {
                            debug!(
                                "requested transactions are successfully \
                                transmitted to the peer node"
                            );
                        }
                        Err(err) => {
                            debug!(
                                "Failed to send requested tx,
                                err: {}",
                                err,
                            );
                        }
                    }
                }
                Msg::TxSyn(h) => {
                    println!("Received the requested txs");
                }
                Msg::HandshakeSyn(_) => {
                    warn!("Peer has sent invalid type message, type: HandshakeSyn");
                }
                Msg::HandshakeAck(_) => {
                    warn!("Peer has sent invalid type message, type: HandshakeAck");
                }
                Msg::TxHashSyn(_) => {
                    println!("May I?");
                    return;
                }
            },
            Err(err) => {
                warn!("Failed to parse the msg, err: {}", err);
                return;
            }
        },
        None => {
            return;
        }
    };
}
