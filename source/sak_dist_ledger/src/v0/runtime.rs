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
        Runtime {
            sync_pool,
            ledger_event_tx,
        }
    }

    pub(crate) async fn run(&self) {}
}
