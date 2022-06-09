use crate::machine::Machine;
use futures::{stream::SplitStream, SinkExt, StreamExt};
use log::{info, warn};
use sak_p2p_trpt::{Connection, Msg, SyncTxHash};
use tokio::sync::RwLockWriteGuard;

pub(crate) async fn handle_msg<'a>(
    msg: Msg,
    machine: &Machine,
    conn: &'a mut RwLockWriteGuard<'_, Connection>,
) {
    match msg {
        // Msg::SyncTx(h) => {
        //     info!("Discovered transactions inserted into tx pool",);

        //     machine.blockchain.insert_into_pool(h.txs).await;
        // }
        Msg::SyncTxHash(sync_tx_hash) => {
            info!(
                "Found sink request will be inserted after hash value \
                comparison.",
            );

            let v = vec![String::from("power")];

            conn.socket
                .send(Msg::RequestTxs(SyncTxHash { tx_hashes: v }))
                .await
                .expect("request txs should be sent");

            // conn.socket.next();

            // machine.blockchain.compare_with_my_pool(h.tx_hashs).await;
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
