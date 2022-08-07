use super::{
    miner::Miner,
    peer_node::PeerNode,
    task::{NodeTask, NodeTaskQueue},
};
use crate::machine::Machine;
use futures::Future;
use sak_p2p_peertable::PeerTable;
use sak_task_queue::{TaskQueue, TaskQueue2};
use std::{pin::Pin, sync::Arc};

pub(crate) struct LocalNode {
    pub(crate) peer_table: Arc<PeerTable>,
    pub(crate) machine: Arc<Machine>,
    pub(crate) miner: bool,
    pub(crate) mine_interval: Option<u64>,
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

            // let task_queue = NodeTaskQueue::init(None).await;
            // TaskQueue2::init(10, f).await;
            let task_queue = TaskQueue::new(10);

            let mut peer_node = PeerNode {
                peer,
                bc_event_rx,
                machine,
                task_queue,
            };

            tokio::spawn(async move {
                peer_node.run().await;
            });
        }
    }
}
