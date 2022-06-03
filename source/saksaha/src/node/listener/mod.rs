use futures::{stream::SplitStream, StreamExt};
use log::warn;
use logger::twarn;
use p2p_peer_table::Peer;
use p2p_transport::Connection;
use p2p_transport::Msg;
use p2p_transport::P2PCodec;
use std::sync::Arc;
use tokio::{net::TcpStream, sync::RwLock};
use tokio_util::codec::Framed;

pub(super) struct PeerListener {}

impl PeerListener {
    pub(super) async fn start_listening(
        // peer: Arc<RwLock<Peer>>
        // socket_rx: &mut SplitStream<Framed<TcpStream, P2PCodec>>,
        conn: &mut Connection,
    ) {
        // let peer_lock = peer.write().await;
        // let mut socket_rx_lock =
        //     peer_lock.transport.conn.socket_rx.write().await;

        // let mut socket_rx_lock = socket_rx.write().await;
        loop {
            let sync = match conn.socket_rx.next().await {
                Some(maybe_msg) => match maybe_msg {
                    Ok(msg) => {
                        if let Msg::Sync(s) = msg {
                            s
                        } else {
                            warn!("Msg arrived not SYNC type");

                            continue;
                        }
                    }
                    Err(err) => {
                        warn!(
                            "Message parse fail, corrupt SYNC message, err: {}",
                            err,
                        );
                        continue;
                    }
                },
                None => {
                    warn!("Message parse fail, corrupt SYNC message",);

                    continue;
                }
            };

            println!("sync msg: {}", sync.value);
        }
    }
}
