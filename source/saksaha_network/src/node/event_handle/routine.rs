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

pub(in crate::node) struct LedgerEventRoutine {
    pub ledger_event_rx: Receiver<DistLedgerEvent>,
    pub machine: Arc<Machine>,
    pub node_task_queue: Arc<TaskQueue<NodeTask>>,
}

impl LedgerEventRoutine {
    pub async fn run(&mut self) {
        loop {
            let ev = match self.ledger_event_rx.recv().await {
                Ok(e) => e,
                Err(err) => {
                    error!("Error receiving ledger event, err: {}", err);

                    continue;
                }
            };

            debug!("Handling ledger event: {:?}", ev);

            let event_handle_res = match ev {
                DistLedgerEvent::TxPoolStat(new_tx_hashes) => {
                    event_handle::handle_tx_pool_stat(
                        &self.machine,
                        new_tx_hashes,
                        &self.node_task_queue,
                    )
                    .await
                }
                DistLedgerEvent::NewBlocks(new_blocks) => {
                    event_handle::handle_new_blocks_ev(
                        &self.machine,
                        new_blocks,
                        &self.node_task_queue,
                    )
                    .await
                }
            };

            if let Err(err) = event_handle_res {
                warn!("Error handling ledger event, err: {}", err);
            }
        }
    }
}
