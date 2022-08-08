use super::msg_handle;
use super::task::NodeTask;
use super::task::NodeTaskQueue;
use crate::{machine::Machine, node::event_handle};
use futures::SinkExt;
use futures::StreamExt;
use log::{debug, warn};
use sak_dist_ledger::DistLedgerEvent;
use sak_p2p_peertable::{Peer, PeerStatus};
use sak_p2p_transport::{BlockHashSynMsg, Msg};
use sak_task_queue::TaskQueue;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::broadcast::Receiver;

pub(in crate::node) struct PeerNode {
    pub peer: Arc<Peer>,
    pub bc_event_rx: Receiver<DistLedgerEvent>,
    pub machine: Arc<Machine>,
    pub task_queue: TaskQueue<NodeTask>,
}

impl PeerNode {
    pub(crate) async fn run(&mut self) {
        debug!(
            "Peer is registered as a peer node. Starting the routine, \
            public_key : {}",
            self.peer.get_public_key_short()
        );

        let task_pusher = self.task_queue.new_pusher();

        // tokio::spawn(async move {
        //     println!("task push after 5 seconds");

        //     tokio::time::sleep(Duration::from_secs(5)).await;

        //     let t = NodeTask::Hello;

        //     let _ = task_pusher.push_back(t).await;
        // });

        loop {
            let mut conn = &mut self.peer.transport.conn.write().await;
            let public_key = self.peer.get_public_key_short();

            tokio::select! {
                Ok(ev) = self.bc_event_rx.recv() => {
                    println!("111111111, pub_key: {}", self.peer.get_public_key_short());
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
                    println!("--end 111111111, pub_key: {}", self.peer.get_public_key_short());
                },
                // Ok(task) = self.task_queue.pop_front() => {
                //     let blocks = self.machine
                //         .blockchain
                //         .dist_ledger
                //         .apis
                //         .get_entire_block_info_list()
                //         .await
                //         .unwrap_or(vec![]);

                //     match conn
                //         .socket
                //         .send(Msg::BlockHashSyn(
                //             BlockHashSynMsg {
                //                 new_blocks: blocks
                //             }
                //         ))
                //         .await
                //     {
                //         Ok(_) => {
                //             debug!("Sending BlockHashSyn",);
                //         }
                //         Err(err) => {
                //             warn!("Failed to BlockHashSyn, err: {}", err,);
                //         }
                //     };
                // },
                maybe_msg = conn
                    // .socket
                    .next_msg() => {
                    println!("2222222222, pub_key: {}", self.peer.get_public_key_short());
                    match maybe_msg {
                        Some(maybe_msg) => match maybe_msg {
                            Ok(msg) => {
                                let _ = msg_handle::handle_msg(
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

                            self.peer.set_status(PeerStatus::Disconnected)
                                .await;

                            return;
                        }
                    };
                    println!("---end 2222222222, pub_key: {}", self.peer.get_public_key_short());
                }
            };
        }
    }

    pub(crate) async fn run_hello(&mut self) {
        debug!(
            "Peer is registered as a peer node. Say hello, \
            public_key : {}",
            self.peer.get_public_key_short()
        );

        let peer_clone = self.peer.clone();
        let machine_clone = self.machine.clone();

        let _ = tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(2)).await;

            let mut conn = peer_clone.transport.conn.write().await;

            let blocks = machine_clone
                .blockchain
                .dist_ledger
                .apis
                .get_entire_block_info_list()
                .await
                .unwrap_or(vec![]);

            match conn
                // .socket
                .send(Msg::BlockHashSyn(BlockHashSynMsg { new_blocks: blocks }))
                .await
            {
                Ok(_) => {
                    debug!("Sending BlockHashSyn",);
                }
                Err(err) => {
                    warn!("Failed to BlockHashSyn, err: {}", err,);
                }
            };
        })
        .await;
    }
}
