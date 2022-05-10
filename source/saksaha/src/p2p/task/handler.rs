use crate::p2p::task::P2PTask;
use logger::{tdebug, terr, twarn};
use p2p_active_calls::CallGuard;
use p2p_transport::connection::Connection;
use p2p_transport_handshake::ops::{handshake, HandshakeInitArgs};
use tokio::net::TcpStream;

pub(crate) async fn run(task: P2PTask) {
    match task {
        P2PTask::InitiateHandshake {
            addr_guard,
            host_state,
        } => {
            let active_calls = &host_state.p2p_active_calls;
            let known_addr = addr_guard.get_known_addr();
            let endpoint = known_addr.p2p_endpoint();

            let call_guard = {
                match active_calls.get(&endpoint).await {
                    Some(call) => {
                        twarn!(
                            "saksaha",
                            "p2p",
                            "Call to initiate handshake is abandoned \
                            since we are already in a call, call: {}",
                            call,
                        );

                        return;
                    }
                    None => {
                        active_calls.insert_outbound(endpoint.clone()).await;

                        CallGuard {
                            endpoint: endpoint.clone(),
                            active_calls: active_calls.clone(),
                        }
                    }
                }
            };

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

            let mut conn = match TcpStream::connect(&endpoint).await {
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
                        "p2p_transport",
                        "handshake",
                        "(caller) Made a connection to destination, \
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
                call_guard,
                addr_guard,
                p2p_port: host_state.p2p_port,
                p2p_identity: host_state.p2p_identity.clone(),
                p2p_peer_table: host_state.p2p_peer_table.clone(),
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
