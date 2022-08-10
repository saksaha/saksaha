use super::{
    miner::Miner,
    peer_node::PeerNode,
    task::{runtime, NodeRuntimeCtx, NodeTask},
};
use crate::machine::Machine;
use futures::Future;
use sak_p2p_peertable::PeerTable;
use sak_task_queue::{TaskQueue, TaskRuntime};
use std::{pin::Pin, sync::Arc};

pub(crate) struct LocalNode {
    pub(crate) peer_table: Arc<PeerTable>,
    pub(crate) machine: Arc<Machine>,
    pub(crate) miner: bool,
    pub(crate) mine_interval: Option<u64>,
    pub(crate) node_task_min_interval: Option<u64>,
}

impl LocalNode {
    pub(crate) async fn run(&self) {
        let machine = self.machine.clone();
        let mine_interval = self.mine_interval.clone();

        if self.miner {
            tokio::spawn(async move {
                let mut miner = Miner::init(machine, mine_interval);

                miner.run().await;
            });
        }

        let node_task_queue = Arc::new(TaskQueue::new(100));
        let node_task_queue_clone = node_task_queue.clone();
        let node_task_min_interval = self.node_task_min_interval.clone();
        let node_runtime_ctx = NodeRuntimeCtx {
            peer_table: self.peer_table.clone(),
        };

        tokio::spawn(async move {
            let node_task_runtime = runtime::create_node_task_runtime(
                node_task_queue_clone,
                node_task_min_interval,
                node_runtime_ctx,
            );

            // node_task_runtime.run().await;
        });

        let peer_it = self.peer_table.new_iter();
        let mut peer_it_lock = peer_it.write().await;

        loop {
            let machine = self.machine.clone();

            let peer = match peer_it_lock.next().await {
                Ok(p) => p.clone(),
                Err(_) => continue,
            };

            let bc_event_rx = {
                let rx = machine
                    .blockchain
                    .dist_ledger
                    .bc_event_tx
                    .clone()
                    .read()
                    .await
                    .subscribe();

                rx
            };

            let mut peer_node = PeerNode {
                peer,
                bc_event_rx,
                machine,
                node_task_queue: node_task_queue.clone(),
            };

            tokio::spawn(async move {
                peer_node.run().await;
            });
        }
    }
}
