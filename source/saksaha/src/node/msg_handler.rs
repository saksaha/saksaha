use crate::machine::Machine;
use futures::{SinkExt, StreamExt};
use log::{info, warn};
use sak_p2p_trpt::{Msg, TxHashSync, UpgradedConnection};
use tokio::sync::RwLockWriteGuard;

pub(crate) async fn handle_msg<'a>(
    public_key: &str,
    msg: Msg,
    machine: &Machine,
    conn: &'a mut RwLockWriteGuard<'_, UpgradedConnection>,
) {
    let txs_to_request = match msg {
        Msg::TxHashSyn(tx_hash_sync) => {
            info!("Handle TxHashSyn msg, src public_key: {}", public_key);

            let txs_to_request = machine
                .blockchain
                .dledger
                .get_tx_pool_diff(tx_hash_sync.tx_hashes)
                .await;

            txs_to_request
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
            tx_hashes: txs_to_request,
        }))
        .await
    {
        Ok(_) => {}
        Err(err) => {
            warn!("Failed to send requested tx, err: {}", err,);
        }
    };

    match conn.socket.next().await {
        Some(maybe_msg) => match maybe_msg {
            Ok(msg) => match msg {
                Msg::TxSyn(h) => {
                    info!("Handling TxSyn msg, src public_key: {}", public_key);

                    machine.blockchain.dledger.insert_into_pool(h.txs).await;
                }
                other_msg => {
                    warn!(
                        "Received an invalid type message, msg: {:?}",
                        other_msg,
                    );
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
