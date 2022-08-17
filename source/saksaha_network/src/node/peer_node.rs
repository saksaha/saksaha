use super::task;
use super::{msg_handle, SaksahaNodeError};
use crate::node::task::NodeTask;
use crate::{
    machine::Machine,
    node::event_handle::{self, LedgerEventRoutine},
};
use log::{debug, error, warn};
use sak_p2p_peertable::{Peer, PeerStatus};
use sak_task_queue::TaskQueue;
use std::sync::Arc;
use std::time::Duration;

pub(in crate::node) struct PeerNode {
    pub peer: Arc<Peer>,
    pub machine: Arc<Machine>,
    pub node_task_min_interval: Duration,
}

impl PeerNode {
    pub(crate) async fn run(self) -> Result<(), SaksahaNodeError> {
        debug!(
            "Peer is registered as a peer node. Starting the routine, \
            public_key : {}",
            self.peer.get_public_key_short()
        );

        let node_task_queue = Arc::new(TaskQueue::new(100));

        {
            // Ledger event routine
            let ledger_event_rx = {
                let rx = self
                    .machine
                    .blockchain
                    .dist_ledger
                    .ledger_event_tx
                    .clone()
                    .read()
                    .await
                    .subscribe();

                rx
            };

            let mut ledger_event_routine = LedgerEventRoutine {
                ledger_event_rx,
                machine: self.machine.clone(),
                node_task_queue: node_task_queue.clone(),
            };

            tokio::spawn(async move {
                ledger_event_routine.run().await;
            });
        }

        {
            // Late sync routine
            let machine_clone = self.machine.clone();

            let new_blocks = machine_clone
                .blockchain
                .dist_ledger
                .apis
                .get_entire_block_info_list()
                .await
                .unwrap_or(vec![]);

            // tokio::spawn(async move {
            node_task_queue
                .push_back(NodeTask::SendBlockHashSyn { new_blocks })
                .await?;

            // let conn_lock = self.peer.get_transport().conn.write().await;

            // let task = node_task_queue.pop_front().await?;

            // task::handle_task(
            //     task,
            //     &node_task_queue,
            //     conn_lock,
            //     &self.machine,
            // )
            // .await;
            // });
        }

        loop {
            let mut conn_lock = self.peer.get_transport().conn.write().await;

            tokio::select! {
                task = node_task_queue.pop_front() => {
                    let task = task?;

                    task::handle_task(task,
                        &node_task_queue, conn_lock, &self.machine).await;
                },
                (maybe_msg, _) = conn_lock.next_msg() => {
                    match maybe_msg {
                        Some(maybe_msg) => match maybe_msg {
                            Ok(msg) => {
                                let _ = msg_handle::handle_msg(
                                    msg,
                                    &self.machine,
                                    conn_lock,
                                    &node_task_queue,
                                    &self.peer,
                                )
                                .await;
                            }
                            Err(err) => {
                                warn!("Failed to parse the msg, err: {}", err);
                            }
                        },
                        None => {
                            warn!("Peer has ended the connection");

                            self.peer.set_peer_status(
                                PeerStatus::Disconnected,
                            ).await;

                            return Err(
                                format!("Peer has ended the connection, \
                                    her_public_key: {}",
                                    self.peer.get_public_key_short()
                                )
                                .into());
                        }
                    };

                }
            }
        }
    }
}
//     pub(crate) async fn run_hello(&mut self) {
//         debug!(
//             "Peer is registered as a peer node. Say hello, \
//             public_key : {}",
//             self.peer.get_public_key_short()
//         );

//         let peer_clone = self.peer.clone();
//         let machine_clone = self.machine.clone();

//         let _ = tokio::spawn(async move {
//             tokio::time::sleep(Duration::from_secs(2)).await;

//             let mut conn = peer_clone.get_transport().conn.write().await;

//             let blocks = machine_clone
//                 .blockchain
//                 .dist_ledger
//                 .apis
//                 .get_entire_block_info_list()
//                 .await
//                 .unwrap_or(vec![]);

//             match conn
//                 .send(Msg::BlockHashSyn(BlockHashSynMsg { new_blocks: blocks }))
//                 .await
//             {
//                 Ok(_) => {
//                     debug!("Sending BlockHashSyn",);
//                 }
//                 Err(err) => {
//                     warn!("Failed to BlockHashSyn, err: {}", err,);
//                 }
//             };
//         })
//         .await;
//     }
// }
