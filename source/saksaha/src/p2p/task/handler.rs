use crate::p2p::task::P2PTask;
use logger::{tdebug, terr, twarn};
use p2p_transport::Connection;
use p2p_transport_ops::handshake::{self, HandshakeInitArgs};
use tokio::net::TcpStream;

pub(crate) async fn run(task: P2PTask) {
    match task {
        P2PTask::InitiateHandshake {
            addr_guard,
            identity,
            peer_table,
        } => {
            let addr = addr_guard.addr.clone();
            let mut addr_lock = addr.write_owned().await;
            let known_addr = &mut addr_lock.known_addr;

            let peer_slot_guard = match peer_table
                .get_mapped_peer_lock(&known_addr.public_key_str)
                .await
            {
                Some(_) => {
                    twarn!(
                        "saksaha",
                        "p2p",
                        "Peer is already mapped, dropping, public_key_str: {}",
                        &known_addr.public_key_str,
                    );

                    return;
                }
                None => match peer_table.get_empty_slot().await {
                    Ok(s) => s,
                    Err(err) => {
                        terr!(
                            "saksaha",
                            "p2p",
                            "Cannot reserve an empty peer node. Dropping \
                            initiate handshake task, err: {}",
                            err,
                        );

                        return;
                    }
                },
            };

            let endpoint = known_addr.p2p_endpoint();

            if utils_net::is_my_endpoint(identity.p2p_port, &endpoint) {
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
                    let c = match Connection::new(s) {
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
                        c.socket_addr,
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
                identity,
                peer_table,
                peer_slot_guard,
                addr_lock,
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
