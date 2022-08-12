use super::msg_handle;
use super::task::NodeTask;
use crate::{
    machine::Machine,
    node::{
        event_handle::{self, LedgerEventRoutine},
        task::NodeTaskHandler,
    },
};
use log::{debug, error, warn};
use sak_dist_ledger::DistLedgerEvent;
use sak_p2p_peertable::{Peer, PeerStatus};
use sak_p2p_transport::{BlockHashSynMsg, Msg};
use sak_task_queue::{TaskQueue, TaskRuntime};
use std::sync::Arc;
use tokio::sync::broadcast::Receiver;

pub(in crate::node) struct PeerNode {
    pub peer: Arc<Peer>,
    pub machine: Arc<Machine>,
    // pub node_task_queue: Arc<TaskQueue<NodeTask>>,
    pub node_task_min_interval: Option<u64>,
}

impl PeerNode {
    pub(crate) async fn run(self) {
        debug!(
            "Peer is registered as a peer node. Starting the routine, \
            public_key : {}",
            self.peer.get_public_key_short()
        );

        let public_key = self.peer.get_public_key_short();

        let node_task_queue = Arc::new(TaskQueue::new(100));
        let node_task_min_interval = self.node_task_min_interval.clone();
        // let peer_register_min_interval =
        //     Duration::from_millis(PEER_REGISTER_MIN_INTERVAL);

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
            // Node task routine
            let node_task_handler = Box::new(NodeTaskHandler {
                // peer_table: self.peer_table.clone(),
            });

            let task_runtime = TaskRuntime::new(
                node_task_queue.clone(),
                node_task_min_interval,
                node_task_handler,
            );

            tokio::spawn(async move {
                task_runtime.run().await;
            });
        }

        loop {
            let mut conn_lock = self.peer.get_transport().conn.write().await;

            let maybe_msg = conn_lock.next_msg().await;

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

                    self.peer.set_peer_status(PeerStatus::Disconnected).await;

                    return;
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
