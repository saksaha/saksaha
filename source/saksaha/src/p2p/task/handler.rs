use crate::p2p::task::P2PTask;
use logger::{tdebug, terr, twarn};
use p2p_transport_ops::handshake::{self, HandshakeInitArgs};

pub(crate) async fn run(task: P2PTask) {
    match task {
        P2PTask::InitiateHandshake {
            addr,
            identity,
            peer_table,
        } => {
            let handshake_init_args = HandshakeInitArgs {
                addr,
                identity,
                peer_table,
            };

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
