use super::ServerError;
use futures::StreamExt;
use logger::{tdebug, twarn};
use p2p_discovery::AddrTable;
use p2p_identity::Identity;
use p2p_peer_table::PeerTable;
use p2p_transport::{Connection, Handshake, Msg};
use p2p_transport_ops::handshake::{self, HandshakeRecvArgs};
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
        addr_table: Arc<AddrTable>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        match conn.socket_rx.next().await {
            Some(maybe_msg) => match maybe_msg {
                Ok(msg) => match msg {
                    Msg::HandshakeSyn(handshake) => {
                        handle_handshake_syn_msg(
                            handshake, conn, identity, peer_table, addr_table,
                        )
                        .await
                    }
                    _ => Err(format!(
                        "Message of this type is not expected at \
                                this stage",
                    )
                    .into()),
                },
                Err(err) => {
                    Err(format!("Error parsing message, err: {}", err).into())
                }
            },
            None => Ok(()),
        }
    }
}

impl Drop for Handler {
    fn drop(&mut self) {
        self.conn_semaphore.add_permits(1);
    }
}

async fn handle_handshake_syn_msg(
    handshake: Handshake,
    conn: Connection,
    identity: Arc<Identity>,
    peer_table: Arc<PeerTable>,
    addr_table: Arc<AddrTable>,
) -> Result<(), ServerError> {
    let addr = match addr_table
        .get_mapped_addr(&handshake.src_public_key_str)
        .await
    {
        Some(a) => a,
        None => {
            return Err(format!(
            "Cannot find addr out of addr_table for the handshake candidate",
        )
            .into());
        }
    };

    let handshake_recv_args = HandshakeRecvArgs {
        handshake_syn: handshake,
        identity,
        peer_table,
        addr,
    };

    match handshake::receive_handshake(handshake_recv_args, conn).await {
        Ok(_) => return Ok(()),
        Err(err) => {
            return Err(err.into());
        }
    };
}
