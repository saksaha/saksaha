use crate::{
    machine::Machine,
    node::{event_handle, task::NodeTask},
};
use log::{debug, error, warn};
use sak_dist_ledger::DistLedgerEvent;
use sak_p2p_peertable::{Peer, PeerStatus};
use sak_p2p_transport::{BlockHashSynMsg, Msg};
use sak_task_queue::TaskQueue;
use std::sync::Arc;
use tokio::sync::broadcast::Receiver;

pub(in crate::node) struct BlockchainEventRoutine {
    pub bc_event_rx: Receiver<DistLedgerEvent>,
    // public_key: String,
    pub machine: Arc<Machine>,
    pub node_task_queue: Arc<TaskQueue<NodeTask>>,
}

impl BlockchainEventRoutine {
    pub async fn run(&mut self) {
        loop {
            let ev = match self.bc_event_rx.recv().await {
                Ok(e) => e,
                Err(err) => {
                    error!("Error receiving bc event, err: {}", err);

                    continue;
                }
            };

            // println!(
            //     "111111111, peer pub_key: {}, ev: {:?}",
            //     self.peer.get_public_key_short(),
            //     ev
            // );

            match ev {
                DistLedgerEvent::NewBlocks(new_blocks) => {
                    event_handle::handle_new_blocks_ev(
                        // &self.public_key,
                        // &mut conn_lock,
                        &self.machine,
                        &new_blocks,
                        &self.node_task_queue,
                    )
                    .await;
                }
                DistLedgerEvent::TxPoolStat(new_tx_hashes) => {
                    event_handle::handle_tx_pool_stat(
                        // &self.public_key,
                        // &mut conn_lock,
                        &self.machine,
                        &new_tx_hashes,
                        &self.node_task_queue,
                    )
                    .await;
                }
            };

            // println!(
            //     "--end 111111111, pub_key: {}",
            //     self.peer.get_public_key_short()
            // );
        }
    }
}
