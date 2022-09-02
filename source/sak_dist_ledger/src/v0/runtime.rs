use crate::{DistLedgerEvent, SyncPool};
use std::sync::Arc;
use tokio::sync::{broadcast::Sender, RwLock};

pub struct Runtime {
    sync_pool: Arc<SyncPool>,
    ledger_event_tx: Arc<Sender<DistLedgerEvent>>,
}

impl Runtime {
    pub(crate) fn init(
        sync_pool: Arc<SyncPool>,
        ledger_event_tx: Arc<Sender<DistLedgerEvent>>,
    ) -> Runtime {
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
