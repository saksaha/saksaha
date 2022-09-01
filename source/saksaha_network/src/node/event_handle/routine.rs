use crate::{
    machine::Machine,
    node::{event_handle, task::NodeTask},
};
use log::{debug, error, warn};
use sak_dist_ledger::DistLedgerEvent;
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

            let event_handle_res = match ev {
                DistLedgerEvent::TxPoolStat(new_tx_hashes) => {
                    self.node_task_queue
                        .push_back(NodeTask::SendTxHashSyn {
                            tx_hashes: new_tx_hashes,
                        })
                        .await
                }
                DistLedgerEvent::NewBlocks(new_blocks) => {
                    self.node_task_queue
                        .push_back(NodeTask::SendBlockHashSyn {
                            new_blocks: new_blocks.clone(),
                        })
                        .await
                }
            };

            if let Err(err) = event_handle_res {
                warn!("Error handling ledger event, err: {}", err);
            }
        }
    }
}
