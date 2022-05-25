use futures::StreamExt;
use logger::{tdebug, twarn};
use p2p_identity::Identity;
use p2p_peer::PeerTable;
use p2p_transport::{Connection, Msg};
use p2p_transport_ops::handshake::{
    self, HandshakeRecvArgs, HandshakeRecvError,
};
use std::sync::Arc;
use tokio::sync::Semaphore;

pub(super) struct Handler {
    pub(crate) conn_semaphore: Arc<Semaphore>,
}

impl Handler {
    pub(super) async fn run(
        &mut self,
        mut conn: Connection,
        identity: Arc<Identity>,
        peer_table: Arc<PeerTable>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        match conn.socket_rx.next().await {
            Some(maybe_msg) => match maybe_msg {
                Ok(msg) => {
                    match msg {
                        Msg::HandshakeSyn(handshake) => {
                            let handshake_recv_args = HandshakeRecvArgs {
                                handshake_syn: handshake,
                                identity,
                                peer_table,
                            };

                            match handshake::receive_handshake(
                                handshake_recv_args,
                                conn,
                            )
                            .await
                            {
                                Ok(_) => (),
                                Err(err) => handle_handshake_recv_error(err),
                            };
                        }
                        _ => {
                            twarn!(
                                "saksaha",
                                "p2p",
                                "Message of this type is not expected at \
                                this stage",
                            );
                        }
                    };
                }

                Err(err) => {
                    twarn!(
                        "saksaha",
                        "p2p",
                        "Error parsing message, err: {}",
                        err
                    );
                }
            },
            None => (),
        };

        Ok(())
    }
}

impl Drop for Handler {
    fn drop(&mut self) {
        self.conn_semaphore.add_permits(1);
    }
}

fn handle_handshake_recv_error(err: HandshakeRecvError) {
    twarn!("saksaha", "p2p", "Handshake recv error, err: {}", err);
}
