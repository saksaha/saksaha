use super::msg_handle;
use super::task::NodeTask;
use crate::{
    machine::Machine,
    node::event_handle::{self, LedgerEventRoutine},
};
use log::{debug, error, warn};
use sak_dist_ledger::DistLedgerEvent;
use sak_p2p_peertable::{Peer, PeerStatus};
use sak_p2p_transport::{BlockHashSynMsg, Msg};
use sak_task_queue::TaskQueue;
use std::sync::Arc;
use tokio::sync::broadcast::Receiver;

pub(in crate::node) struct PeerNode {
    pub peer: Arc<Peer>,
    // pub bc_event_rx: Receiver<DistLedgerEvent>,
    pub machine: Arc<Machine>,
    pub node_task_queue: Arc<TaskQueue<NodeTask>>,
}

impl PeerNode {
    pub(crate) async fn run(self) {
        debug!(
            "Peer is registered as a peer node. Starting the routine, \
            public_key : {}",
            self.peer.get_public_key_short()
        );

        let public_key = self.peer.get_public_key_short();
        let node_task_queue = self.node_task_queue.clone();

        loop {
            let mut conn_lock =
                &mut self.peer.get_transport().conn.write().await;

            tokio::select! {
                Ok(task) = self.node_task_queue.pop_front() => {
                    // let blocks = self.machine
                    //     .blockchain
                    //     .dist_ledger
                    //     .apis
                    //     .get_entire_block_info_list()
                    //     .await
                    //     .unwrap_or(vec![]);

                    // match conn
                    //     .socket
                    //     .send(Msg::BlockHashSyn(
                    //         BlockHashSynMsg {
                    //             new_blocks: blocks
                    //         }
                    //     ))
                    //     .await
                    // {
                    //     Ok(_) => {
                    //         debug!("Sending BlockHashSyn",);
                    //     }
                    //     Err(err) => {
                    //         warn!("Failed to BlockHashSyn, err: {}", err,);
                    //     }
                    // };
                },
                maybe_msg = conn_lock
                    .next_msg() => {
                    println!("2222222222, pub_key: {}",
                        self.peer.get_public_key_short());

                    match maybe_msg {
                        Some(maybe_msg) => match maybe_msg {
                            Ok(msg) => {
                                let _ = msg_handle::handle_msg(
                                    msg,
                                    &self.machine,
                                    &mut conn_lock,
                                    &self.node_task_queue,
                                    &self.peer,
                                ).await;
                            }
                            Err(err) => {
                                warn!("Failed to parse the msg, err: {}", err);
                            }
                        }
                        None => {
                            warn!("Peer has ended the connection");

                            self.peer.set_peer_status(PeerStatus::Disconnected)
                                .await;

                            return;
                        }
                    };

                    println!("---end 2222222222, pub_key: {}",
                        self.peer.get_public_key_short());
                }
            };
        }
    }

    // pub(crate) async fn run_hello(&mut self) {
    //     debug!(
    //         "Peer is registered as a peer node. Say hello, \
    //         public_key : {}",
    //         self.peer.get_public_key_short()
    //     );

    //     let peer_clone = self.peer.clone();
    //     let machine_clone = self.machine.clone();

    //     let _ = tokio::spawn(async move {
    //         tokio::time::sleep(Duration::from_secs(2)).await;

    //         let mut conn = peer_clone.get_transport().conn.write().await;

    //         let blocks = machine_clone
    //             .blockchain
    //             .dist_ledger
    //             .apis
    //             .get_entire_block_info_list()
    //             .await
    //             .unwrap_or(vec![]);

    //         match conn
    //             .send(Msg::BlockHashSyn(BlockHashSynMsg { new_blocks: blocks }))
    //             .await
    //         {
    //             Ok(_) => {
    //                 debug!("Sending BlockHashSyn",);
    //             }
    //             Err(err) => {
    //                 warn!("Failed to BlockHashSyn, err: {}", err,);
    //             }
    //         };
    //     })
    //     .await;
    // }
}
