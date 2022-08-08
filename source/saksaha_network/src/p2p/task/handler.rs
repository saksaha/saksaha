use crate::p2p::task::P2PTask;
use log::{debug, error, warn};
use sak_p2p_peertable::{Peer, PeerStatus};
use sak_p2p_transport::{
    handshake::{self, HandshakeInitArgs},
    Conn,
};
use std::sync::Arc;
use tokio::{net::TcpStream, sync::RwLock};

pub(crate) async fn run(task: P2PTask) {
    match task {
        P2PTask::InitiateHandshake {
            addr,
            identity,
            peer_table,
        } => {
            let known_addr = &addr.known_addr;

            if let Some(p) =
                peer_table.get_mapped_peer(&known_addr.public_key_str).await
            {
                debug!(
                    "Peer already mapped, public_key: {}",
                    p.get_public_key_short()
                );

                return;
            }

            let peer_slot_guard = match peer_table.get_empty_slot().await {
                Ok(p) => p,
                Err(err) => {
                    error!(
                        "Fatal error. Empty slot is not available in the \
                        peer table, err: {}",
                        err
                    );

                    return;
                }
            };

            let endpoint = known_addr.get_p2p_endpoint();

            if sak_utils_net::is_my_endpoint(identity.p2p_port, &endpoint) {
                warn!(
                    "Cannot make a request to myself, abandoning handshake \
                    init task, endopint: {}",
                    &endpoint,
                );
                return;
            }

            let conn_id = sak_crypto::rand();

            let conn = match TcpStream::connect(&endpoint).await {
                Ok(s) => {
                    let c = match Conn::new(s, conn_id, true) {
                        Ok(c) => {
                            debug!(
                                "Successfully connected to endpoint: {}, \
                                conn_id: {}",
                                &endpoint, conn_id
                            );

                            c
                        }
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
