use crate::p2p::task::P2PTask;
use chrono::{Duration, Utc};
use logger::{tdebug, terr, twarn};
use p2p_addr::AddrStatus;
use p2p_discovery::AddrVal;
use p2p_peer::{PeerSlot, PeerStatus};
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

            let mut known_addr = match &mut addr_lock.val {
                AddrVal::Known(k) => k,
                AddrVal::Unknown(_) => {
                    terr!(
                        "saksaha",
                        "p2p",
                        "Addr table has invalid entry (not known)",
                    );

                    return;
                }
            };

            let peer_slot = match peer_table
                .get_mapped_peer_lock(&known_addr.public_key_str)
                .await
            {
                Some(mut peer) => {
                    if let PeerStatus::HandshakeSuccess { at } = peer.status {
                        let now = Utc::now();
                        if now.signed_duration_since(at) < Duration::seconds(60)
                        {
                            tdebug!(
                                "saksaha",
                                "p2p",
                                "Handshake has been done recently, dropping \
                                the task (InitiateHandshake)",
                            );

                            if let Some(_) = &peer.addr_guard {
                                known_addr.status = AddrStatus::Invalid {
                                    err: format!(
                                        "Handshake is done recently. \
                                            HSInit dropped"
                                    ),
                                };
                            } else {
                                peer.addr_guard = Some(addr_guard);
                            }

                            return;
                        }
                    }
                    PeerSlot::Peer(peer)
                }
                None => match peer_table.get_empty_slot().await {
                    Ok(s) => PeerSlot::Slot(s),
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
                peer_slot,
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
