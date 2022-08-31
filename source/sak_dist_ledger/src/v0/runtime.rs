use crate::{DistLedgerEvent, SyncPool};
use log::{debug, error, warn};
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};
use tokio::sync::{broadcast::Sender, RwLock};

const TX_SYNC_INTERVAL: u64 = 2000;
const BLOCK_SYNC_INTERVAL: u64 = 2000;

pub struct Runtime {
    sync_pool: Arc<SyncPool>,
    ledger_event_tx: Arc<Sender<DistLedgerEvent>>,
    tx_sync_interval: Duration,
    block_sync_interval: Duration,
}

impl Runtime {
    pub(crate) fn init(
        sync_pool: Arc<SyncPool>,
        ledger_event_tx: Arc<Sender<DistLedgerEvent>>,
        tx_sync_interval: Option<u64>,
        block_sync_interval: Option<u64>,
    ) -> Runtime {
        let tx_sync_interval = match tx_sync_interval {
            Some(i) => Duration::from_millis(i.into()),
            None => Duration::from_millis(TX_SYNC_INTERVAL),
        };

        let block_sync_interval = match block_sync_interval {
            Some(i) => Duration::from_millis(i.into()),
            None => Duration::from_millis(BLOCK_SYNC_INTERVAL),
        };

        Runtime {
            sync_pool,
            ledger_event_tx,
            tx_sync_interval,
            block_sync_interval,
        }
    }

    pub(crate) async fn run(&self) {
        // let tx_sync_routine = TxSyncRoutine {
        //     sync_pool: self.sync_pool.clone(),
        //     tx_sync_interval: self.tx_sync_interval,
        //     ledger_event_tx: self.ledger_event_tx.clone(),
        // };

        // tokio::spawn(async move {
        //     tx_sync_routine.run().await;
        // });

        // let block_sync_routine = BlockSyncRoutine {
        //     sync_pool: self.sync_pool.clone(),
        //     block_sync_interval: self.block_sync_interval,
        //     ledger_event_tx: self.ledger_event_tx.clone(),
        // };

        // tokio::spawn(async move {
        //     block_sync_routine.run().await;
        // });
    }

    // pub(crate) async fn new(
    //     sync_pool: &SyncPool,
    //     bc_event_tx: Arc<RwLock<Sender<DistLedgerEvent>>>,
    // ) -> {
    //     //
    // }
}

struct TxSyncRoutine {
    sync_pool: Arc<SyncPool>,
    bc_event_tx: Arc<RwLock<Sender<DistLedgerEvent>>>,
    tx_sync_interval: Duration,
}

impl TxSyncRoutine {
    pub(crate) async fn run(&self) {
        // loop {
        //     let time_since = SystemTime::now();

        //     let new_tx_hashes = self.sync_pool.drain_new_tx_hashes().await;

        //     if new_tx_hashes.len() > 0 {
        //         match self
        //             .bc_event_tx
        //             .clone()
        //             .write()
        //             .await
        //             .send(DistLedgerEvent::TxPoolStat(new_tx_hashes))
        //         {
        //             Ok(_) => (),
        //             Err(err) => {
        //                 warn!(
        //                     "No active tx sync routine receiver handle to \
        //                         sync tx event, \
        //                     err: {}",
        //                     err
        //                 );
        //             }
        //         };
        //     }

        //     sak_utils_time::wait_until_min_interval(
        //         time_since,
        //         self.tx_sync_interval,
        //     )
        //     .await;
        // }
    }
}

struct BlockSyncRoutine {
    sync_pool: Arc<SyncPool>,
    bc_event_tx: Arc<RwLock<Sender<DistLedgerEvent>>>,
    block_sync_interval: Duration,
}

impl BlockSyncRoutine {
    pub(crate) async fn run(&self) {
        // loop {
        //     let time_since = SystemTime::now();

        //     let new_blocks = self.sync_pool.drain_new_blocks().await;

        //     if new_blocks.len() > 0 {
        //         let ev = DistLedgerEvent::NewBlocks(new_blocks);
        //         let ev_str = ev.to_string();

        //         match self.bc_event_tx.clone().write().await.send(ev) {
        //             Ok(_) => {
        //                 debug!("Ledger event queued, ev: {}", ev_str);
        //             }
        //             Err(err) => {
        //                 error!(
        //                     "Could not queue a new ledger event, err: {}",
        //                     err
        //                 );
        //             }
        //         }
        //     }

        //     sak_utils_time::wait_until_min_interval(
        //         time_since,
        //         self.block_sync_interval,
        //     )
        //     .await;
        // }
    }
}
