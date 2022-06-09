use super::tx_pool::TxPool;
use crate::BlockchainEvent;
use log::error;
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};
use tokio::sync::mpsc::Sender;

const TX_LOOP_SYNC_INTERVAL: u64 = 3000;

pub(crate) struct Runtime {
    tx_pool: Arc<TxPool>,
    bc_event_tx: Arc<Sender<BlockchainEvent>>,
}

impl Runtime {
    pub(crate) fn init(
        tx_pool: Arc<TxPool>,
        bc_event_tx: Arc<Sender<BlockchainEvent>>,
    ) -> Runtime {
        Runtime {
            tx_pool,
            bc_event_tx,
        }
    }

    pub(crate) async fn run(&self) {
        let tx_pool_sync_interval =
            Duration::from_millis(TX_LOOP_SYNC_INTERVAL);

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
                tx_pool_sync_interval,
            )
            .await;
        }
    }
}
