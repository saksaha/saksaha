use super::DistLedgerEvent;
use crate::Consensus;
use crate::LedgerDB;
use crate::LedgerError;
use crate::SyncPool;
use sak_crypto::hasher::MiMC;
use sak_crypto::MerkleTree;
use sak_ledger_cfg::CM_TREE_DEPTH;
use sak_logger::info;
use sak_types::BlockCandidate;
use sak_vm_interface::ContractProcess;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::broadcast::Sender;

const BLOCKCHAIN_EVENT_QUEUE_CAPACITY: usize = 32;

pub struct SakLedger {
    pub ledger_event_tx: Arc<Sender<DistLedgerEvent>>,
    pub ledger_db: LedgerDB,
    pub sync_pool: Arc<SyncPool>,
    pub merkle_tree: MerkleTree,
    pub hasher: MiMC,
    pub consensus: Box<dyn Consensus + Send + Sync>,
    pub contract_processor: Box<dyn ContractProcess + Send + Sync>,
}

pub struct SakLedgerArgs {
    pub tx_sync_interval: Option<u64>,
    pub genesis_block: Option<BlockCandidate>,
    pub consensus: Box<dyn Consensus + Send + Sync>,
    pub block_sync_interval: Option<u64>,
    pub ledger_path: PathBuf,
    pub contract_processor: Box<dyn ContractProcess + Send + Sync>,
}

impl SakLedger {
    pub async fn init(ledger_args: SakLedgerArgs) -> Result<Self, LedgerError> {
        let SakLedgerArgs {
            tx_sync_interval,
            genesis_block,
            consensus,
            block_sync_interval,
            ledger_path,
            contract_processor,
        } = ledger_args;

        let ledger_db = LedgerDB::init(&ledger_path).await?;

        let ledger_event_tx = {
            let (tx, _rx) = broadcast::channel(BLOCKCHAIN_EVENT_QUEUE_CAPACITY);

            Arc::new(tx)
        };

        let sync_pool = {
            let tx = ledger_event_tx.clone();

            let p = SyncPool::new(tx, tx_sync_interval, block_sync_interval);

            Arc::new(p)
        };

        let hasher = MiMC::new();

        let merkle_tree = MerkleTree::new(CM_TREE_DEPTH as u32);

        let ledger = SakLedger {
            ledger_event_tx,
            ledger_db,
            sync_pool,
            merkle_tree,
            hasher,
            consensus,
            contract_processor,
        };

        if let Some(bc) = genesis_block {
            ledger.insert_genesis_block(bc).await?;
        }

        let latest_height = match ledger.ledger_db.get_latest_block_height()? {
            Some(h) => h.to_string(),
            None => "No block yet".to_string(),
        };

        info!(
            "Initialized Blockchain, latest added height (none if genesis \
            block has not been inserted): {}",
            latest_height,
        );

        Ok(ledger)
    }
}
