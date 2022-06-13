use super::{miner::Miner, peer_node::PeerNode};
use crate::{
    machine::Machine,
    node::{event_handle, msg_handler},
};
use futures::StreamExt;
use log::{debug, warn};
use sak_blockchain::BlockchainEvent;
use sak_p2p_ptable::{PeerStatus, PeerTable};
use std::sync::Arc;

pub(crate) struct LocalNode {
    pub(crate) peer_table: Arc<PeerTable>,
    pub(crate) machine: Arc<Machine>,
    pub(crate) miner: bool,
    pub(crate) mine_interval: Option<u64>,
}

impl LocalNode {
    pub(crate) async fn run(&self) {
        // let peer_node_rt = PeerNodeRoutine {};
        // tokio::spawn(async {
        //     peer_node_rt.run();
        // });

        let machine = self.machine.clone();
        let mine_interval = self.mine_interval.clone();
        tokio::spawn(async move {
            let miner = Miner {
                machine,
                mine_interval,
            };

            miner.run().await;
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
                // run_node_routine(peer_node, machine).await;
            });
        }
    }
}
