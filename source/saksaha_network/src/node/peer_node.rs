use super::msg_handle;
use crate::{machine::Machine, node::event_handle};
use futures::SinkExt;
use futures::StreamExt;
use log::{debug, warn};
use sak_dist_ledger::DistLedgerEvent;
use sak_p2p_peertable::{Peer, PeerStatus};
use sak_p2p_transport::{BlockHashSynMsg, Msg};
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

        {
            let peer_clone = self.peer.clone();
            let machine_clone = self.machine.clone();

            tokio::spawn(async move {
                let mut conn = peer_clone.transport.conn.write().await;
                // event_handle::handle_new_peers_ev(
                //     public_key,
                //     &mut conn.socket_tx,
                //     &machine_clone,
                // )
                // .await;

                let blocks = machine_clone
                    .blockchain
                    .dist_ledger
                    .apis
                    .get_entire_block_info_list()
                    .await
                    .unwrap_or(vec![]);

                conn.socket_tx
                    .send(Msg::BlockHashSyn(BlockHashSynMsg {
                        new_blocks: blocks,
                    }))
                    .await
            });
        }

        loop {
            let conn = &mut self.peer.transport.conn.write().await;
            let public_key = self.peer.get_public_key_short();

            tokio::select! {
                Ok(ev) = self.bc_event_rx.recv() => {
                    match ev {
                        DistLedgerEvent::NewBlocks(new_blocks) => {
                            event_handle::handle_new_blocks_ev(
                                public_key,
                                &mut conn.socket_tx,
                                &self.machine,
                                // height,
                                new_blocks,
                            ).await;
                        },
                        DistLedgerEvent::TxPoolStat(new_tx_hashes) => {
                            event_handle::handle_tx_pool_stat(
                                public_key,
                                &mut conn.socket_tx,
                                &self.machine,
                                new_tx_hashes,
                            ).await;
                        },
                    };
                },
                maybe_msg = conn.socket_rx.next() => {
                    match maybe_msg {
                        Some(maybe_msg) => match maybe_msg {
                            Ok(msg) => {
                                let _ = msg_handle::handle_msg(
                                    msg,
                                    public_key,
                                    &self.machine,
                                    &mut conn.socket_tx,

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
