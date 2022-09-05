use super::DistLedgerEvent;
use crate::Consensus;
use crate::DistLedgerApis;
use crate::LedgerDB;
use crate::LedgerError;
use crate::Runtime;
use crate::SyncPool;
use log::info;
use sak_crypto::MerkleTree;
use sak_dist_ledger_meta::CM_TREE_DEPTH;
use sak_proofs::Hasher;
use sak_types::BlockCandidate;
use sak_vm::VM;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::{broadcast::Sender, RwLock};

const BLOCKCHAIN_EVENT_QUEUE_CAPACITY: usize = 32;

pub struct DistLedger {
    pub apis: DistLedgerApis,
    pub ledger_event_tx: Arc<Sender<DistLedgerEvent>>,
    runtime: Arc<Runtime>,
}

pub struct DistLedgerArgs {
    pub app_prefix: String,
    pub tx_sync_interval: Option<u64>,
    pub genesis_block: Option<BlockCandidate>,
    pub consensus: Box<dyn Consensus + Send + Sync>,
    pub block_sync_interval: Option<u64>,
}

impl DistLedger {
    pub async fn init(
        dist_ledger_args: DistLedgerArgs,
    ) -> Result<DistLedger, LedgerError> {
        let DistLedgerArgs {
            app_prefix,
            tx_sync_interval,
            genesis_block,
            consensus,
            block_sync_interval,
        } = dist_ledger_args;

        let ledger_db = LedgerDB::init(&app_prefix).await?;

        let vm = VM::init()?;

        let ledger_event_tx = {
            let (tx, _rx) = broadcast::channel(BLOCKCHAIN_EVENT_QUEUE_CAPACITY);

            Arc::new(tx)
        };

        let sync_pool = {
            let tx = ledger_event_tx.clone();

            let p = SyncPool::new(tx);

            Arc::new(p)
        };

        let runtime = {
            let r = Runtime::init(
                sync_pool.clone(),
                ledger_event_tx.clone(),
                tx_sync_interval,
                block_sync_interval,
            );

            Arc::new(r)
        };

        let hasher = Hasher::new();

        let merkle_tree = MerkleTree::new(CM_TREE_DEPTH as u32);

        let apis = DistLedgerApis {
            ledger_db,
            vm,
            sync_pool,
            merkle_tree,
            hasher,
            consensus,
        };

        let dist_ledger = DistLedger {
            apis,
            ledger_event_tx,
            runtime,
        };

        if let Some(bc) = genesis_block {
            dist_ledger.apis.insert_genesis_block(bc).await?;
        }

        let latest_height =
            dist_ledger.apis.ledger_db.get_latest_block_height()?;

        info!(
            "Initialized Blockchain, latest height (none if genesis \
                block has not been inserted): {:?}",
            latest_height,
        );

        Ok(dist_ledger)
    }

    pub async fn run(&self) {
        let runtime = self.runtime.clone();

        tokio::spawn(async move {
            runtime.run().await;
        });
    }
}
