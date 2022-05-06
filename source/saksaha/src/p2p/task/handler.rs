use crate::p2p::task::P2PTask;
use logger::terr;
use logger::twarn;
use p2p_active_calls::ActiveCalls;
use p2p_transport::handshake::{self, HandshakeInitArgs};
use std::sync::Arc;
use std::time::Duration;

pub(crate) async fn run(task: P2PTask) {
    match task {
        P2PTask::InitiateHandshake {
            addr_guard,
            host_state,
        } => {
            let active_calls = &host_state.p2p_active_calls;
            let addr = addr_guard.get_value();
            let endpoint = addr.p2p_endpoint();

            let _call_guard = {
                match active_calls.get(&endpoint).await {
                    Some(call) => {
                        twarn!(
                            "saksaha",
                            "p2p",
                            "Call to initiate handshake is abandoned \
                            since we are already in a call, call: {}",
                            call,
                        );
                    }
                    None => {
                        active_calls.insert_outbound(endpoint.clone()).await;

                        CallGuard {
                            endpoint,
                            active_calls: active_calls.clone(),
                        };
                    }
                }
            };

            let handshake_init_args = HandshakeInitArgs {
                addr: addr.clone(),
                p2p_port: host_state.p2p_port,
                p2p_identity: host_state.p2p_identity.clone(),
            };

            tokio::spawn(async move {
                println!("moved, will wait for 5 seconds");

                tokio::time::sleep(Duration::from_secs(5)).await;

                addr_guard;
            });

            match handshake::initiate_handshake(handshake_init_args).await {
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

struct CallGuard {
    endpoint: String,
    active_calls: Arc<ActiveCalls>,
}

impl Drop for CallGuard {
    fn drop(&mut self) {
        match self.active_calls.delayed_remove(self.endpoint.clone()) {
            Ok(_) => (),
            Err(err) => {
                terr!("saksaha", "p2p", "Call removal error, err: {}", err);
            }
        }
    }
}
