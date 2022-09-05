use super::{miner::Miner, peer_node::PeerNode};
use crate::machine::Machine;
use log::{info, warn};
use sak_p2p_peertable::PeerTable;
use std::{sync::Arc, time::Duration};
use tokio::time::Instant;

const PEER_REGISTER_MIN_INTERVAL: u64 = 1000;
const NODE_TASK_INTERVAL: u64 = 1000;

pub(crate) struct LocalNode {
    pub peer_table: Arc<PeerTable>,
    pub machine: Arc<Machine>,
    pub miner: bool,
    pub mine_interval: Option<u64>,
    pub node_task_interval: Duration,
    pub peer_register_interval: Duration,
}

impl LocalNode {
    pub fn new(
        peer_table: Arc<PeerTable>,
        machine: Arc<Machine>,
        miner: bool,
        mine_interval: Option<u64>,
        node_task_interval: Option<u64>,
        peer_register_interval: Option<u64>,
    ) -> LocalNode {
        let node_task_interval = match node_task_interval {
            Some(i) => Duration::from_millis(i),
            None => Duration::from_millis(NODE_TASK_INTERVAL),
        };

        let peer_register_interval = match peer_register_interval {
            Some(i) => Duration::from_millis(i),
            None => Duration::from_millis(PEER_REGISTER_MIN_INTERVAL),
        };

        info!(
            "local node is initialized, node_task_interval: {:?},\
            peer_register_interval: {:?}",
            node_task_interval, peer_register_interval,
        );

        LocalNode {
            peer_table,
            machine,
            miner,
            mine_interval,
            node_task_interval,
            peer_register_interval,
        }
    }

    pub(crate) async fn run(&self) {
        let machine = self.machine.clone();

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
                let now = Instant::now();

                let machine = self.machine.clone();

                let peer = match peer_queue_iter_lock.next().await {
                    Ok(p) => p.clone(),
                    Err(_) => continue,
                };

                let peer_node = PeerNode {
                    peer_table: self.peer_table.clone(),
                    peer: peer.clone(),
                    machine,
                    node_task_min_interval: self.node_task_interval.clone(),
                };

                tokio::spawn(async move {
                    let res = peer_node.run().await;

                    if let Err(err) = res {
                        warn!("Peer routine is terminated, err: {}", err);
                    }
                });

                tokio::time::sleep_until(now + self.peer_register_interval)
                    .await;
            }
        }
    }
}
