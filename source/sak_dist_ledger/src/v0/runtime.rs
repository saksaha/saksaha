use crate::{v0::tx_pool::TxPool, DLedgerEvent};
use log::error;
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};
use tokio::sync::{broadcast::Sender, RwLock};

const TX_POOL_SYNC_INTERVAL: u64 = 1000;

pub struct Runtime {
    tx_pool: Arc<TxPool>,
    bc_event_tx: Arc<RwLock<Sender<DLedgerEvent>>>,
    tx_pool_sync_interval: Duration,
}

impl Runtime {
    pub(crate) fn init(
        tx_pool: Arc<TxPool>,
        bc_event_tx: Arc<RwLock<Sender<DLedgerEvent>>>,
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

            // println!("new_tx_hashes :{:?}", new_tx_hashes);

            if new_tx_hashes.len() > 0 {
                match self
                    .bc_event_tx
                    .clone()
                    .write()
                    .await
                    .send(DLedgerEvent::TxPoolStat(new_tx_hashes))
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
