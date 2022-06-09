use crate::machine::Machine;
use futures::{stream::SplitStream, SinkExt, StreamExt};
use log::{debug, info, warn};
use sak_p2p_trpt::{Connection, Msg, TxHashSyn};
use tokio::sync::RwLockWriteGuard;

pub(crate) async fn handle_msg<'a>(
    msg: Msg,
    machine: &Machine,
    conn: &'a mut RwLockWriteGuard<'_, Connection>,
) {
    match msg {
        Msg::TxHashSyn(sync_tx_hash) => {
            info!(
                "Found sync request will be inserted after hash value \
                comparison",
            );

            let req_hashes = machine
                .blockchain
                .compare_with_pool(sync_tx_hash.tx_hashes)
                .await;

            match conn
                .socket
                .send(Msg::TxHashAck(TxHashSyn {
                    tx_hashes: req_hashes,
                }))
                .await
            {
                Ok(_) => {
                    debug!(
                        "requested tx hashes are successfully \
                        transmitted to the peer node"
                    );
                }
                Err(err) => {
                    debug!("Failed to send requested tx, err: {}", err,);
                }
            };
        }
        Msg::TxSyn(h) => {
            info!("Received the requested txs");

            machine.blockchain.insert_into_pool(h.txs).await;
        }
        Msg::HandshakeSyn(_) => {
            warn!("Peer has sent invalid type message, type: HandshakeSyn");
        }
        Msg::HandshakeAck(_) => {
            warn!("Peer has sent invalid type message, type: HandshakeAck");
        }
        _ => {}
    };
}
