use crate::machine::Machine;
use futures::{SinkExt, StreamExt};
use log::{info, warn};
use sak_p2p_trpt::{Connection, Msg, TxHashSync};
use tokio::sync::RwLockWriteGuard;

pub(crate) async fn handle_msg<'a>(
    msg: Msg,
    machine: &Machine,
    conn: &'a mut RwLockWriteGuard<'_, Connection>,
) {
    let req_hashes = match msg {
        Msg::TxHashSyn(tx_hash_sync) => {
            info!(
                "Found sync request will be inserted after hash value \
                comparison, got msg type: TxHashSyn",
            );

            let req_hashes = machine
                .blockchain
                .get_tx_pool_diff(tx_hash_sync.tx_hashes)
                .await;

            if req_hashes.is_empty() {
                warn!("No difference, no need to request");
                return;
            }

            req_hashes
        }
        Msg::TxSyn(_) => {
            warn!("Peer has sent invalid type message, type: TxSyn");
            return;
        }
        Msg::HandshakeSyn(_) => {
            warn!("Peer has sent invalid type message, type: HandshakeSyn");
            return;
        }
        Msg::HandshakeAck(_) => {
            warn!("Peer has sent invalid type message, type: HandshakeAck");
            return;
        }
        _ => return,
    };

    match conn
        .socket
        .send(Msg::TxHashAck(TxHashSync {
            tx_hashes: req_hashes,
        }))
        .await
    {
        Ok(_) => {
            info!(
                "Request the tx hashes to peer node, send msg type: TxHashAck"
            );
        }
        Err(err) => {
            warn!("Failed to send requested tx, err: {}", err,);
        }
    };

    match conn.socket.next().await {
        Some(maybe_msg) => match maybe_msg {
            Ok(msg) => match msg {
                Msg::TxSyn(h) => {
                    info!(
                        "Received the requested txs, got msg type: \
                                TxSyn"
                    );

                    machine.blockchain.insert_into_pool(h.txs).await;
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
