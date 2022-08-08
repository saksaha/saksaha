use chrono::Utc;
use futures::StreamExt;
use log::{error, warn};
use sak_p2p_discovery::AddrTable;
use sak_p2p_id::Identity;
use sak_p2p_peertable::{Peer, PeerStatus, PeerTable};
use sak_p2p_transport::{
    handshake::{self, HandshakeRecvArgs},
    Conn, Handshake, Msg,
};
use std::sync::Arc;
use tokio::sync::{RwLock, Semaphore};

pub(super) struct Handler {
    pub(crate) conn_semaphore: Arc<Semaphore>,
}

impl Handler {
    pub(super) async fn run(
        &mut self,
        mut conn: Conn,
        identity: Arc<Identity>,
        peer_table: Arc<PeerTable>,
        addr_table: Arc<AddrTable>,
    ) {
        let handshake_recv_args = HandshakeRecvArgs { identity };

        let peer_slot_guard = match peer_table.get_empty_slot().await {
            Ok(s) => s,
            Err(err) => {
                error!(
                    "Empty slot is not available in the peer table, err: {}",
                    err
                );
                return;
            }
        };

        let (transport, her_public_key_str) =
            match handshake::receive_handshake(handshake_recv_args, conn).await
            {
                Ok(t) => t,
                Err(err) => {
                    warn!("Error receiving handshake, err: {}", err);
                    return;
                }
            };

        let addr = match addr_table.get_mapped_addr(&her_public_key_str).await {
            Some(a) => a,
            None => {
                warn!(
                    "Cannot find addr out of addr_table for the \
                handshake candidate"
                );

                return;
            }
        };

        let peer = {
            let p = Peer {
                transport,
                p2p_port: addr.known_addr.p2p_port,
                public_key_str: addr.known_addr.public_key_str.clone(),
                addr,
                status: RwLock::new(PeerStatus::HandshakeSuccess {
                    at: Utc::now(),
                }),
                peer_slot_guard,
            };

            Arc::new(p)
        };

        if let Err(err) = peer_table.insert_mapping(peer).await {
            warn!("Error inserting peer mapping, err: {}", err);
            return;
        }
    }
}

impl Drop for Handler {
    fn drop(&mut self) {
        self.conn_semaphore.add_permits(1);
    }
}
