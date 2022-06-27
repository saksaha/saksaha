use super::tx_pool::TxPool;
use super::DLedgerEvent;
use crate::Consensus;
use crate::LedgerDB;
use crate::Runtime;
use colored::Colorize;
use log::{error, info, warn};
use sak_types::BlockCandidate;
use sak_vm::VM;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::{broadcast::Sender, RwLock};

const BLOCKCHAIN_EVENT_QUEUE_CAPACITY: usize = 32;

pub struct DistLedger<C> {
    pub(crate) ledger_db: LedgerDB,
    pub(crate) tx_pool: Arc<TxPool>,
    pub bc_event_tx: Arc<RwLock<Sender<DLedgerEvent>>>,
    pub(crate) vm: VM,
    consensus: C,
    runtime: Arc<Runtime>,
}

pub struct DistLedgerArgs<C> {
    pub app_prefix: String,
    pub tx_pool_sync_interval: Option<u64>,
    pub genesis_block: Option<BlockCandidate>,
    pub consensus: C,
    // pub consensus: Consensu
}

impl<C> DistLedger<C> {
    pub async fn init(
        blockchain_args: DistLedgerArgs<C>,
    ) -> Result<DistLedger<C>, String> {
        let DistLedgerArgs {
            app_prefix,
            tx_pool_sync_interval,
            genesis_block,
            consensus,
        } = blockchain_args;

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

        let tx_pool = {
            let t = TxPool::new();

            Arc::new(t)
        };

        let bc_event_tx = {
            let (tx, _rx) = broadcast::channel(BLOCKCHAIN_EVENT_QUEUE_CAPACITY);

            Arc::new(RwLock::new(tx))
        };

        let runtime = {
            let r = Runtime::init(
                tx_pool.clone(),
                bc_event_tx.clone(),
                tx_pool_sync_interval,
            );

            Arc::new(r)
        };

        let dist_ledger = DistLedger {
            ledger_db,
            tx_pool: tx_pool.clone(),
            vm,
            bc_event_tx,
            consensus,
            runtime,
        };

        if let Some(bc) = genesis_block {
            dist_ledger.insert_genesis_block(&bc).await?;
        }

        info!("Initialized Blockchain");

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
        genesis_block: &BlockCandidate,
    ) -> Result<String, String> {
        let persisted_gen_block_hash = if let Some(b) =
            match self.get_block_by_height(&String::from("0")).await {
                Ok(b) => b,
                Err(err) => return Err(err.to_string()),
            } {
            let block_hash = b.get_hash().to_string();

            info!(
                "Genesis block is already persisted, block_hash: {}",
                block_hash,
            );

            block_hash
        } else {
            info!("Genesis block not found, writing");

            let b = match self.write_block(&genesis_block).await {
                Ok(b) => b,
                Err(err) => return Err(err.to_string()),
            };

            b
        };

        let (gen_block, _) = genesis_block.extract();
        let gen_block_hash = gen_block.get_hash();

        if gen_block_hash != &persisted_gen_block_hash {
            return Err(format!(
                "Not identical genesis block. Hardwird genesis \
            block may have been tampered, gen_block: {}, persisted: {}",
                &gen_block_hash, &persisted_gen_block_hash,
            )
            .into());
        }

        info!("Genesis block hash: {}", gen_block_hash.yellow());

        Ok(persisted_gen_block_hash.to_string())
    }
}
