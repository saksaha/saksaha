use super::tx_pool::TxPool;
use super::DLedgerEvent;
use crate::Database;
use crate::Runtime;
use log::{error, info, warn};
use sak_types::Block;
use sak_types::BlockCandidate;
use sak_vm::VM;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::{broadcast::Sender, RwLock};

const BLOCKCHAIN_EVENT_QUEUE_CAPACITY: usize = 32;

pub struct DistLedger {
    pub(crate) database: Database,
    pub(crate) tx_pool: Arc<TxPool>,
    // pub(crate) gen_block_hash: Option<String>,
    pub bc_event_tx: Arc<RwLock<Sender<DLedgerEvent>>>,
    vm: VM,
    runtime: Arc<Runtime>,
}

pub struct DistLedgerArgs {
    pub app_prefix: String,
    pub tx_pool_sync_interval: Option<u64>,
    // pub genesis_block: BlockCandidate,
}

pub struct DistLedgerInitState {
    // genesis block insertion result
    // contract_1_addr,
    // contract_2_addr,
    // contract_3_addr,
    pub gen_block_insert_result: Vec<String>,
}

impl DistLedger {
    pub async fn init(
        blockchain_args: DistLedgerArgs,
    ) -> Result<DistLedger, String> {
        let DistLedgerArgs {
            app_prefix,
            tx_pool_sync_interval,
            // genesis_block,
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

        let mut dist_ledger = DistLedger {
            database,
            tx_pool: tx_pool.clone(),
            vm,
            bc_event_tx,
            runtime,
            // gen_block_hash: None,
        };

        // let gen_block_hash =
        //     dist_ledger.insert_genesis_block(genesis_block).await?;

        // dist_ledger.gen_block_hash = Some(gen_block_hash);

        info!("Initialized Blockchain");

        Ok(dist_ledger)
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
        let (block, _) = genesis_block.extract();

        let gen_block_hash = block.get_hash().to_owned();

        if let Ok(b) = self.get_block(&gen_block_hash).await {
            info!(
                "Genesis block has already been inserted. \
                This might not be the first time you launch the node, \
                block: {:?}",
                b
            );

            return Ok(gen_block_hash);
        }

        {
            if let Err(err) = self.write_block(genesis_block).await {
                error!("Cannot create genesis block, err: {}", err);
            };
        }

        Ok(gen_block_hash)
    }

    pub async fn get_gen_block(&self) -> Result<Block, String> {
        self.get_block_by_height(String::from("0")).await
    }

    pub fn get_vm(&self) -> &VM {
        &self.vm
    }
}
