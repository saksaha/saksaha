use super::{miner::Miner, peer_node::PeerNode};
use crate::machine::Machine;
use log::info;
use sak_p2p_id::{Credential, Identity};
use sak_p2p_ptable::PeerTable;
use std::sync::Arc;

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

        info!("Running LocalNode, miner: {}", self.miner);

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
            };

            tokio::spawn(async move {
                peer_node.run().await;
            });
        }
    }
}
