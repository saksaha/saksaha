use super::{
    event_handle::LedgerEventRoutine, miner::Miner, peer_node::PeerNode,
    task::NodeTask,
};
use crate::machine::Machine;
use log::warn;
use sak_p2p_peertable::{Peer, PeerTable};
use sak_task_queue::TaskQueue;
use std::{
    pin::Pin,
    sync::Arc,
    time::{Duration, SystemTime},
};
use tokio::{sync::RwLock, time::Instant};

const PEER_REGISTER_MIN_INTERVAL: u64 = 1000;
const NODE_TASK_INTERVAL: u64 = 1000;

pub(crate) struct LocalNode {
    pub peer_table: Arc<PeerTable>,
    pub machine: Arc<Machine>,
    pub miner: bool,
    pub mine_interval: Option<u64>,
    pub node_task_min_interval: Option<u64>,
    pub peer_register_interval: Option<u64>,
}

impl LocalNode {
    pub fn new(
        peer_table: Arc<PeerTable>,
        machine: Arc<Machine>,
        miner: bool,
        mine_interval: Option<u64>,
        node_task_min_interval: Option<u64>,
        peer_register_interval: Option<u64>,
    ) -> LocalNode {
        LocalNode {
            peer_table,
            machine,
            miner,
            mine_interval,
            node_task_min_interval,
            peer_register_interval,
        }
    }

    pub(crate) async fn run(&self) {
        let machine = self.machine.clone();
        let node_task_interval = match self.node_task_min_interval {
            Some(i) => Duration::from_millis(i),
            None => Duration::from_millis(NODE_TASK_INTERVAL),
        };

        // Miner routine
        if self.miner {
            let mine_interval = self.mine_interval;
            tokio::spawn(async move {
                let mut miner = Miner::init(machine, mine_interval);

                miner.run().await;
            });
        }

        {
            let peer_queue_iter = self.peer_table.peer_queue_iter();
            let mut peer_queue_iter_lock = peer_queue_iter.write().await;

            loop {
                println!("loop");

                let now = Instant::now();

                let machine = self.machine.clone();

                let peer = match peer_queue_iter_lock.next().await {
                    Ok(p) => p.clone(),
                    Err(_) => continue,
                };

                let peer_node = PeerNode {
                    peer: peer.clone(),
                    machine,
                    node_task_min_interval: self.node_task_min_interval.clone(),
                };

                tokio::spawn(async move {
                    let res = peer_node.run().await;

                    if let Err(err) = res {
                        warn!("Peer routine is terminated, err: {}", err);
                    }
                });

                tokio::time::sleep_until(now + node_task_interval).await;
            }
        }
    }
}
