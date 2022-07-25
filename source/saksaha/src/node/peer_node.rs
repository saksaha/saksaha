use crate::node::msg_handler;
use crate::{machine::Machine, node::event_handle};
use futures::{SinkExt, StreamExt};
use log::{debug, error, info, warn};
use sak_dist_ledger::DistLedgerEvent;
use sak_p2p_ptable::Peer;
use sak_p2p_ptable::{PeerStatus, PeerTable};
use std::sync::Arc;
use tokio::sync::broadcast::Receiver;

pub(in crate::node) struct PeerNode {
    pub(crate) peer: Arc<Peer>,
    pub(crate) bc_event_rx: Receiver<DistLedgerEvent>,
    pub(crate) machine: Arc<Machine>,
}

impl PeerNode {
    pub(crate) async fn run(&mut self) {
        debug!(
            "Peer is registered as a peer node. Starting the routine, \
            public_key : {}",
            self.peer.get_public_key_short()
        );

        loop {
            let mut conn = &mut self.peer.transport.conn.write().await;
            let public_key = self.peer.get_public_key_short();

            tokio::select! {
                Ok(ev) = self.bc_event_rx.recv() => {
                    match ev {
                        DistLedgerEvent::NewBlocks(new_blocks) => {
                            event_handle::handle_new_blocks_ev(
                                public_key,
                                &mut conn,
                                &self.machine,
                                // height,
                                new_blocks,
                            ).await;
                        },
                        DistLedgerEvent::TxPoolStat(new_tx_hashes) => {
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
                    println!("22, msg: {:?}", maybe_msg);

                    match maybe_msg {
                        Some(maybe_msg) => match maybe_msg {
                            Ok(msg) => {
                                let _ = msg_handler::handle_msg(
                                    msg,
                                    public_key,
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

                            self.peer.set_status(PeerStatus::Disconnected).await;

                            return;
                        }
                    };
                }
            };
        }
    }
}
