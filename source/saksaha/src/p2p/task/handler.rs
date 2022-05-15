use crate::p2p::task::P2PTask;
use logger::{tdebug, terr, twarn};
use p2p_peer::PeerNode;
use p2p_transport::connection::Connection;
use p2p_transport_handshake::ops::{handshake, HandshakeInitArgs};
use tokio::{net::TcpStream, sync::OwnedRwLockWriteGuard};

pub(crate) async fn run(task: P2PTask) {
    match task {
        P2PTask::InitiateHandshake {
            addr_guard,
            host_state,
        } => {
            let known_addr = match addr_guard.get_known_addr().await {
                Ok(a) => a,
                Err(err) => {
                    terr!(
                        "saksaha",
                        "p2p",
                        "Addr table has invalid entry (not known), \
                                err: {}",
                        err
                    );
                    return;
                }
            };

            let p2p_peer_table = host_state.p2p_peer_table.clone();

            let (peer_node_lock, peer_node) = match p2p_peer_table
                .get_mapped_node_lock(&known_addr.public_key_str)
                .await
            {
                Some((peer_node_lock, peer_node)) => {
                    println!("initiate handshake, record already exists");

                    (peer_node_lock, peer_node)
                }
                None => match p2p_peer_table.get_empty_node_lock().await {
                    Some(n) => n,
                    None => {
                        twarn!(
                            "saksaha",
                            "p2p",
                            "Cannot reserve an empty peer node. Dropping \
                            initiate handshake task."
                        );

                        return;
                    }
                },
            };

            let endpoint = known_addr.p2p_endpoint();

            if utils_net::is_my_endpoint(host_state.p2p_port, &endpoint) {
                twarn!(
                    "saksaha",
                    "p2p",
                    "Cannot make a request to myself, abandoning handshake \
                    init task, endopint: {}",
                    &endpoint
                );

                return;
            }

            let conn = match TcpStream::connect(&endpoint).await {
                Ok(s) => {
                    let (c, peer_addr) = match Connection::new(s) {
                        Ok(c) => c,
                        Err(err) => {
                            terr!(
                                "saksaha",
                                "p2p",
                                "Cannot acquire peer address, err: {}",
                                err,
                            );

                            return;
                        }
                    };

                    tdebug!(
                        "saksaha",
                        "p2p",
                        "(caller) TCP connected to destination, \
                        peer_addr: {:?}",
                        peer_addr,
                    );

                    c
                }
                Err(err) => {
                    terr!(
                        "saksaha",
                        "p2p",
                        "Cannot make a tcp connection to an \
                        endpoint, endpoint: {}, err: {}",
                        &endpoint,
                        err,
                    );

                    return;
                }
            };

            let handshake_init_args = HandshakeInitArgs {
                addr_guard,
                p2p_port: host_state.p2p_port,
                p2p_identity: host_state.p2p_identity.clone(),
                p2p_peer_table: host_state.p2p_peer_table.clone(),
                peer_node_lock,
                peer_node,
            };

            match handshake::initiate_handshake(handshake_init_args, conn).await
            {
                Ok(_) => (),
                Err(err) => {
                    twarn!(
                        "saksaha",
                        "p2p",
                        "Error processing InitiateHandshake, discarding, \
                        err: {}",
                        err,
                    );
                }
            }
        }
    };
}
