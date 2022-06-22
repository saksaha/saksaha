use crate::node::msg_handler;
use crate::{machine::Machine, node::event_handle};
use futures::StreamExt;
use log::{debug, warn};
use sak_blockchain::BlockchainEvent;
use sak_p2p_ptable::Peer;
use sak_p2p_ptable::{PeerStatus, PeerTable};
use std::sync::Arc;
use tokio::sync::broadcast::Receiver;

pub(crate) struct PeerNode {
    pub(crate) peer: Arc<Peer>,
    pub(crate) bc_event_rx: Receiver<BlockchainEvent>,
    pub(in crate::node) machine: Arc<Machine>,
}

impl PeerNode {
    pub(in crate::node) async fn run(&mut self) {
        debug!(
            "Peer is registered as a peer node. Starting the routine, \
            public_key: {}",
            self.peer.public_key_short()
        );

        loop {
            let mut conn = &mut self.peer.transport.conn.write().await;
            let public_key = self.peer.public_key_short();

            tokio::select! {
                Ok(ev) = self.bc_event_rx.recv() => {
                    match ev {
                        BlockchainEvent::TxPoolStat(new_tx_hashes) => {
                            event_handle::handle_tx_pool_stat(
                                public_key,
                                &mut conn,
                                &self.machine,
                                new_tx_hashes,
                            ).await;
                        },
                    };
                },
                maybe_msg = conn.socket.next() => {
                    match maybe_msg {
                        Some(maybe_msg) => match maybe_msg {
                            Ok(msg) => {
                                let _ = msg_handler::handle_msg(
                                    public_key,
                                    msg,
                                    &self.machine,
                                    &mut conn,
                                ).await;
                            }
                            Err(err) => {
                                warn!("Failed to parse the msg, err: {}", err);
                            }
                        }
                        None => {
                            warn!("Peer has ended the connection");

                            let mut status_lock = self.peer.status.write()
                                .await;

                            *status_lock = PeerStatus::Disconnected;
                            return;
                        }
                    };
                }
            }
        }
    }
}
