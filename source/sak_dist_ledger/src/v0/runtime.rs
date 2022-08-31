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
        }
    }

    pub(crate) async fn run(&self) {}
}

struct TxSyncRoutine {
    sync_pool: Arc<SyncPool>,
    bc_event_tx: Arc<RwLock<Sender<DistLedgerEvent>>>,
}

impl TxSyncRoutine {
    pub(crate) async fn run(&self) {}
}

struct BlockSyncRoutine {
    sync_pool: Arc<SyncPool>,
    bc_event_tx: Arc<RwLock<Sender<DistLedgerEvent>>>,
}

impl BlockSyncRoutine {
    pub(crate) async fn run(&self) {}
}
