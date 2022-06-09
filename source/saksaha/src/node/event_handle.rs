use futures::stream::SplitSink;
use futures::{stream::SplitStream, SinkExt, StreamExt};
use log::{debug, warn};
use sak_blockchain::Transaction;
use sak_p2p_trpt::{Connection, Msg, P2PCodec, SyncTx, SyncTxHash};
use tokio::{net::TcpStream, sync::RwLockWriteGuard};
use tokio_util::codec::Framed;

pub(super) async fn handle_tx_pool_stat<'a>(
    conn: &'a mut RwLockWriteGuard<'_, Connection>,
    new_tx_hashes: Vec<String>,
) {
    match conn
        .socket
        .send(Msg::SyncTxHash(SyncTxHash {
            tx_hashes: new_tx_hashes,
        }))
        .await
    {
        Ok(_) => {
            debug!(
                "Incoming tx successfully synced with \
                the peer node"
            );
        }
        Err(err) => {
            debug!(
                "Failed to sync tx with the peer nodes, \
                err: {}",
                err,
            );
        }
    };

    let maybe_msg = conn.socket.next().await;

    match maybe_msg {
        Some(maybe_msg) => match maybe_msg {
            Ok(msg) => {
                println!("11111111111111111111111",);
                // let _ = msg_handler::handle_msg(msg, &machine, &mut conn).await;
            }
            Err(err) => {
                warn!("Failed to parse the msg, err: {}", err);
            }
        },
        None => {
            warn!("33333333333333333");
            return;
        }
    };
}
