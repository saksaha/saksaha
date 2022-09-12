use super::task;
use super::{msg_handle, SaksahaNodeError};
use crate::node::task::NodeTask;
use crate::{
    machine::Machine,
    node::event_handle::{self, LedgerEventRoutine},
};
use log::{debug, error, warn};
use sak_p2p_discovery::Discovery;
use sak_p2p_peertable::{Peer, PeerStatus, PeerTable};
use sak_task_queue::TaskQueue;
use std::sync::Arc;
use std::time::Duration;

pub(in crate::node) struct PeerNode {
    pub peer_table: Arc<PeerTable>,
    pub peer: Arc<Peer>,
    pub machine: Arc<Machine>,
    pub discovery: Arc<Discovery>,
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
            let ledger_event_rx = {
                let rx = self
                    .machine
                    .blockchain
                    .dist_ledger
                    .ledger_event_tx
                    .clone()
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
            // say hello
            let unknown_addrs = self.peer_table.get_peer_addrs().await;

            if !self.peer.is_initiator {
                node_task_queue
                    .push_back(NodeTask::SendHelloSyn { unknown_addrs })
                    .await?
            }
        }

        {
            // Late sync routine
            if let Ok(new_blocks) = self
                .machine
                .blockchain
                .dist_ledger
                .apis
                .get_all_blocks()
                .await
            {
                if new_blocks.len() > 1 {
                    node_task_queue
                        .push_back(NodeTask::SendBlockHashSyn { new_blocks })
                        .await?
                }
            }
        }

        loop {
            let mut conn_lock = self.peer.get_transport().conn.write().await;

            println!(
                "  >> loop next_msg: peer id: {}, ",
                self.peer.get_public_key_short()
            );

            tokio::select! {
                task = node_task_queue.pop_front() => {

                    let task = task?;

                    match task::handle_task(
                        task,
                        &node_task_queue,
                        conn_lock,
                        &self.machine,
                        &self.discovery
                    ).await {
                        Ok(r) => r,
                        Err(err) => {
                            error!(
                                "peer node task handle failed, err: {}",
                                err,
                            );
                        }
                    };
                },
                maybe_msg = conn_lock.next_msg() => {
                    match maybe_msg {
                        Some(msg) => match msg {
                            Ok(m) => {
                                let _ = msg_handle::handle_msg(
                                    m,
                                    &self.machine,
                                    conn_lock,
                                    &node_task_queue,
                                    // &self.peer,
                                    &self.peer_table,
                                    &self.discovery,
                                )
                                .await;
                            }
                            Err(err) => {
                                error!("Failed to parse the msg, err: {}", err);
                            }
                        },
                        None => {
                            self.peer.set_peer_status(
                                PeerStatus::Disconnected,
                            ).await;

                            return Err(
                                format!("Peer has ended the connection, \
                                    conn_id: {}, her_public_key: {}",
                                    conn_lock.get_conn_id(),
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
