use crate::{v0::tx_pool::TxPool, BlockchainEvent};
use log::error;
use sak_task_queue::TaskQueue;
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};
use tokio::sync::mpsc::Sender;

const TX_LOOP_SYNC_INTERVAL: u64 = 3000;

pub struct Runtime {
    tx_pool: Arc<TxPool>,
    // bc_event_tx: Arc<Sender<BlockchainEvent>>,
    pub task_queue: Arc<TaskQueue<BlockchainEvent>>,
    sync_task_interval: Duration,
}

impl Runtime {
    pub(crate) fn init(
        tx_pool: Arc<TxPool>,
        // bc_event_tx: Arc<Sender<BlockchainEvent>>,
        task_queue: Arc<TaskQueue<BlockchainEvent>>,
        sync_task_interval: Option<u16>,
    ) -> Runtime {
        let sync_task_interval = match sync_task_interval {
            Some(i) => Duration::from_millis(i.into()),
            None => Duration::from_millis(TX_LOOP_SYNC_INTERVAL),
        };

        Runtime {
            tx_pool,
            task_queue,
            sync_task_interval,
        }
    }

    pub(crate) async fn run(&self) {
        loop {
            let time_since = SystemTime::now();

            let new_tx_hashes = self.tx_pool.get_new_tx_hashes().await;

            if !new_tx_hashes.is_empty() {
                let task = BlockchainEvent::TxPoolStat(new_tx_hashes);
                match self.task_queue.push_back(task).await {
                    Ok(_) => {}
                    Err(err) => {
                        error!("Error sending blockchain event, err: {}", err);
                    }
                };
            }

            // if !new_tx_hashes.is_empty() {
            //     match self
            //         .bc_event_tx
            //         .send(BlockchainEvent::TxPoolStat(new_tx_hashes))
            //         .await
            //     {
            //         Ok(_) => (),
            //         Err(err) => {
            //             error!("Error sending blockchain event, err: {}", err);
            //         }
            //     };
            // }

            sak_utils_time::wait_until_min_interval(
                time_since,
                self.sync_task_interval,
            )
            .await;
        }
    }
}
