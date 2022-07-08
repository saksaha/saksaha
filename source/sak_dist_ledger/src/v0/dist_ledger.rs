use super::DistLedgerEvent;
use crate::Consensus;
use crate::LedgerDB;
use crate::Runtime;
use crate::SyncPool;
use colored::Colorize;
use log::info;
use sak_types::BlockCandidate;
use sak_vm::VM;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::{broadcast::Sender, RwLock};

const BLOCKCHAIN_EVENT_QUEUE_CAPACITY: usize = 32;

pub struct DistLedger {
    pub(crate) ledger_db: LedgerDB,
    pub(crate) sync_pool: Arc<SyncPool>,
    pub bc_event_tx: Arc<RwLock<Sender<DistLedgerEvent>>>,
    pub(crate) vm: VM,
    pub(crate) consensus: Box<dyn Consensus + Send + Sync>,
    runtime: Arc<Runtime>,
    // pub(crate) merkle_tree: MerkleTree,
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
    ) -> Result<DistLedger, String> {
        let DistLedgerArgs {
            app_prefix,
            tx_sync_interval,
            genesis_block,
            consensus,
            block_sync_interval,
        } = dist_ledger_args;

        let ledger_db = match LedgerDB::init(&app_prefix).await {
            Ok(d) => d,
            Err(err) => {
                return Err(format!(
                    "Error initializing database, err: {}",
                    err,
                ));
            }
        };

        let vm = VM::init()?;

        let sync_pool = {
            let p = SyncPool::new();

            Arc::new(p)
        };

        let bc_event_tx = {
            let (tx, _rx) = broadcast::channel(BLOCKCHAIN_EVENT_QUEUE_CAPACITY);

            Arc::new(RwLock::new(tx))
        };

        let runtime = {
            let r = Runtime::init(
                sync_pool.clone(),
                bc_event_tx.clone(),
                tx_sync_interval,
                block_sync_interval,
            );

            Arc::new(r)
        };

        let dist_ledger = DistLedger {
            ledger_db,
            sync_pool,
            vm,
            bc_event_tx,
            consensus,
            runtime,
            // merkle_tree,
        };

        if let Some(bc) = genesis_block {
            dist_ledger.insert_genesis_block(bc).await?;

            // TODO
            // genesis_block hash check
        }

        let latest_height = {
            let maybe_height =
                match dist_ledger.ledger_db.get_latest_block_height().await {
                    Ok(h) => h,
                    Err(err) => {
                        return Err(format!(
                            "Failed to get latest block height, err: {}",
                            err,
                        ))
                    }
                };

            maybe_height.unwrap_or(0)
        };

        info!(
            "Initialized Blockchain, latest height: {}",
            latest_height.to_string().yellow(),
        );

        Ok(dist_ledger)
    }

    pub async fn run(&self) {
        let runtime = self.runtime.clone();

        tokio::spawn(async move {
            runtime.run().await;
        });
    }

    async fn insert_genesis_block(
        &self,
        genesis_block: BlockCandidate,
    ) -> Result<String, String> {
        let persisted_gen_block_hash = if let Some(b) =
            match self.get_block_by_height(&0).await {
                Ok(b) => b,
                Err(err) => return Err(err.to_string()),
            } {
            let block_hash = b.get_hash().to_string();

            info!(
                "Genesis block is already persisted, block_hash: {}",
                block_hash.green(),
            );

            block_hash
        } else {
            info!("Genesis block not found, writing");

            let b = match self.write_block(Some(genesis_block)).await {
                Ok(b) => b.ok_or(
                    "genesis block should have been written as it \
                        does not exist at the moment",
                )?,
                Err(err) => return Err(err.to_string()),
            };

            b
        };

        Ok(persisted_gen_block_hash.to_string())
    }
}
