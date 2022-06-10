use crate::{v0::tx_pool::TxPool, BlockchainEvent};
use log::error;
use sak_task_queue::TaskQueue;
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};
use tokio::sync::mpsc::Sender;

const TX_POOL_SYNC_INTERVAL: u64 = 3000;

pub struct Runtime {
    tx_pool: Arc<TxPool>,
    bc_event_tx: Arc<Sender<BlockchainEvent>>,
    tx_pool_sync_interval: Duration,
}

impl Runtime {
    pub(crate) fn init(
        tx_pool: Arc<TxPool>,
        bc_event_tx: Arc<Sender<BlockchainEvent>>,
        tx_pool_sync_interval: Option<u64>,
    ) -> Runtime {
        let tx_pool_sync_interval = match tx_pool_sync_interval {
            Some(i) => Duration::from_millis(i.into()),
            None => Duration::from_millis(TX_POOL_SYNC_INTERVAL),
        };

        Runtime {
            tx_pool,
            bc_event_tx,
            tx_pool_sync_interval,
        }
    }

    pub(crate) async fn run(&self) {
        loop {
            let time_since = SystemTime::now();

            let new_tx_hashes = self.tx_pool.get_new_tx_hashes().await;

            if !new_tx_hashes.is_empty() {
                match self
                    .bc_event_tx
                    .send(BlockchainEvent::TxPoolStat(new_tx_hashes))
                    .await
                {
                    Ok(_) => (),
                    Err(err) => {
                        error!("Error sending blockchain event, err: {}", err);
                    }
                };
            }

            sak_utils_time::wait_until_min_interval(
                time_since,
                self.tx_pool_sync_interval,
            )
            .await;
        }
    }
}
