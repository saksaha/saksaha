use super::tx_pool::TxPool;
use super::DLedgerEvent;
use crate::Database;
use crate::Runtime;
use log::{error, info, warn};
use sak_types::BlockCandidate;
use sak_vm::VM;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::{broadcast::Sender, RwLock};

const BLOCKCHAIN_EVENT_QUEUE_CAPACITY: usize = 32;

pub struct DLedger {
    pub(crate) database: Database,
    pub(crate) tx_pool: Arc<TxPool>,
    pub(crate) gen_block_hash: Option<String>,
    pub bc_event_tx: Arc<RwLock<Sender<DLedgerEvent>>>,
    vm: VM,
    runtime: Arc<Runtime>,
}

pub struct DLedgerArgs {
    pub app_prefix: String,
    pub tx_pool_sync_interval: Option<u64>,
    pub genesis_block: BlockCandidate,
}

pub struct DLedgerInitState {
    // genesis block insertion result
    // contract_1_addr,
    // contract_2_addr,
    // contract_3_addr,
    pub gen_block_insert_result: Vec<String>,
}

impl DLedger {
    pub async fn init(
        blockchain_args: DLedgerArgs,
    ) -> Result<(DLedger, DLedgerInitState), String> {
        let DLedgerArgs {
            app_prefix,
            tx_pool_sync_interval,
            genesis_block,
        } = blockchain_args;

        let database = match Database::init(&app_prefix).await {
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

        let mut dist_ledger = DLedger {
            database,
            tx_pool: tx_pool.clone(),
            vm,
            bc_event_tx,
            runtime,
            gen_block_hash: None,
        };

        let gen_block_hash =
            match dist_ledger.insert_genesis_block(genesis_block).await {
                Ok(h) => h,
                Err(err) => {
                    return Err(format!(
                        "Cannot insert genesis block, err: {}",
                        err,
                    ));
                }
            };

        dist_ledger.gen_block_hash = Some(gen_block_hash);

        let dledger_init_state = DLedgerInitState {
            gen_block_insert_result: vec![],
        };

        info!("Initialized Blockchain");

        Ok((dist_ledger, dledger_init_state))
    }

    pub async fn run(&self) {
        let runtime = self.runtime.clone();

        tokio::spawn(async move {
            runtime.run().await;
        });
    }

    pub async fn insert_genesis_block(
        &self,
        genesis_block: BlockCandidate,
    ) -> Result<String, String> {
        let (block, txs) = genesis_block.extract();

        let gen_block_hash = block.get_hash().to_owned();

        match self.get_block(&gen_block_hash).await {
            Ok(_) => {
                warn!("A Genesis block has already been created");
            }
            Err(_) => {
                info!("Build a genesis block");

                if let Err(err) = self.write_block(block).await {
                    error!("Cannot create genesis block, err: {}", err);
                };
            }
        };

        for tx in txs {
            if let Err(err) = self.write_tx(&tx).await {
                error!("Could not write tx of genesis block, err: {}", err);
            }
        }

        Ok(gen_block_hash)
    }

    pub fn get_vm(&self) -> &VM {
        &self.vm
    }
}
