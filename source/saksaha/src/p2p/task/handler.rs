use std::sync::Arc;

use crate::p2p::task::P2PTask;
use log::{debug, warn};
use p2p_peer_table::{Peer, PeerStatus};
use p2p_transport::{
    handshake::{self, HandshakeInitArgs},
    Connection,
};
use tokio::{net::TcpStream, sync::RwLock};
// use p2p_transport_ops::handshake::{self, HandshakeInitArgs};

pub(crate) async fn run(task: P2PTask) {
    match task {
        P2PTask::InitiateHandshake {
            addr,
            identity,
            peer_table,
        } => {
            let known_addr = &addr.known_addr;

            let peer_slot_guard = match peer_table
                .get_mapped_peer(&known_addr.public_key_str)
                .await
            {
                Some(_) => {
                    warn!(
                        "Peer already mapped, public_key: {}",
                        &known_addr.public_key_str,
                    );
                    return;
                }
                None => match peer_table.get_empty_slot().await {
                    Ok(s) => s,
                    Err(_) => {
                        warn!("Empty slot is not available in the peer table");
                        return;
                    }
                },
            };

            let endpoint = known_addr.p2p_endpoint();

            if utils_net::is_my_endpoint(identity.p2p_port, &endpoint) {
                warn!(
                    "Cannot make a request to myself, abandoning handshake \
                    init task, endopint: {}",
                    &endpoint,
                );
                return;
            }

            let conn = match TcpStream::connect(&endpoint).await {
                Ok(s) => {
                    let c = match Connection::new(s) {
                        Ok(c) => c,
                        Err(err) => {
                            warn!("Error creating a connection, err: {}", err);
                            return;
                        }
                    };

                    debug!(
                        "(caller) TCP connected to destination, \
                        peer_addr: {:?}",
                        c.socket_addr,
                    );

                    c
                }
                Err(err) => {
                    warn!(
                        "Error connecting to p2p_endpoint ({}), err: {}",
                        &endpoint, err,
                    );
                    return;
                }
            };

            let handshake_init_args = HandshakeInitArgs {
                identity,
                conn,
                public_key_str: known_addr.public_key_str.clone(),
            };

            let transport = match handshake::initiate_handshake(
                handshake_init_args,
            )
            .await
            {
                Ok(t) => t,
                Err(err) => {
                    warn!(
                        "Error processing InitiateHandshake, discarding, \
                        err: {}",
                        err,
                    );

                    return;
                }
            };

            let peer = {
                let p = Peer {
                    p2p_port: known_addr.p2p_port,
                    public_key_str: known_addr.public_key_str.clone(),
                    addr,
                    transport,
                    status: RwLock::new(PeerStatus::HandshakeInit),
                    peer_slot_guard,
                };

                Arc::new(p)
            };

            if let Err(err) = peer_table.insert_mapping(peer).await {
                warn!("Cannot insert mapping in the peer table, err: {}", err);
            }
        }
    };
}
