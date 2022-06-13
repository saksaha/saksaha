use std::sync::Arc;

use futures::{SinkExt, StreamExt};
use log::{info, warn};
use sak_p2p_ptable::Peer;
use sak_p2p_trpt::{Connection, Msg, TxHashSync, TxSyn};
use tokio::sync::RwLockWriteGuard;

use crate::machine::Machine;

use super::peer_node::PeerNode;

pub(super) async fn handle_tx_pool_stat<'a>(
    public_key: &str,
    conn: &'a mut RwLockWriteGuard<'_, Connection>,
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

    let txs = match conn.socket.next().await {
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
