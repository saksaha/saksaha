use super::{
    event_handle::LedgerEventRoutine,
    miner::Miner,
    peer_node::PeerNode,
    task::{NodeTask, NodeTaskHandler},
};
use crate::machine::Machine;
use sak_p2p_peertable::{Peer, PeerTable};
use sak_task_queue::{TaskQueue, TaskRuntime};
use std::{
    pin::Pin,
    sync::Arc,
    time::{Duration, SystemTime},
};
use tokio::sync::RwLock;

const PEER_REGISTER_MIN_INTERVAL: u64 = 1000;

pub(crate) struct LocalNode {
    pub peer_table: Arc<PeerTable>,
    pub machine: Arc<Machine>,
    pub miner: bool,
    pub mine_interval: Option<u64>,
    pub node_task_min_interval: Option<u64>,
}

impl LocalNode {
    pub fn new(
        peer_table: Arc<PeerTable>,
        machine: Arc<Machine>,
        miner: bool,
        mine_interval: Option<u64>,
        node_task_min_interval: Option<u64>,
    ) -> LocalNode {
        LocalNode {
            peer_table,
            machine,
            miner,
            mine_interval,
            node_task_min_interval,
        }
    }

    pub(crate) async fn run(self) {
        let machine = self.machine.clone();
        let mine_interval = self.mine_interval.clone();
        let node_task_queue = Arc::new(TaskQueue::new(100));
        let node_task_min_interval = self.node_task_min_interval.clone();
        let peer_register_min_interval =
            Duration::from_millis(PEER_REGISTER_MIN_INTERVAL);

        {
            // Miner routine
            if self.miner {
                tokio::spawn(async move {
                    let mut miner = Miner::init(machine, mine_interval);

                    miner.run().await;
                });
            }
        }

        // {
        //     // Node task routine
        //     let node_task_handler = Box::new(NodeTaskHandler {
        //         peer_table: self.peer_table.clone(),
        //     });

        //     let task_runtime = TaskRuntime::new(
        //         node_task_queue.clone(),
        //         node_task_min_interval,
        //         node_task_handler,
        //     );

        //     tokio::spawn(async move {
        //         task_runtime.run().await;
        //     });
        // }

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
            let peer_queue_iter = self.peer_table.peer_queue_iter();
            let mut peer_queue_iter_lock = peer_queue_iter.write().await;

            loop {
                let time_since = SystemTime::now();

                let machine = self.machine.clone();

                let peer = match peer_queue_iter_lock.next().await {
                    Ok(p) => p.clone(),
                    Err(_) => continue,
                };

                let peer_node = PeerNode {
                    peer: peer.clone(),
                    machine,
                    node_task_queue: node_task_queue.clone(),
                };

                tokio::spawn(async move {
                    peer_node.run().await;
                });

                sak_utils_time::wait_until_min_interval(
                    time_since,
                    peer_register_min_interval,
                )
                .await;
            }
        }
    }
}
