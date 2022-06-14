use super::tx_pool::TxPool;
use super::BlockchainEvent;
use crate::Database;
use crate::Runtime;
use log::{info, warn};
use sak_types::{Block, Hashable};
use sak_vm::VM;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::{broadcast::Sender, RwLock};

const BLOCKCHAIN_EVENT_QUEUE_CAPACITY: usize = 32;

pub struct Blockchain {
    pub(crate) database: Database,
    pub(crate) tx_pool: Arc<TxPool>,
    pub bc_event_tx: Arc<RwLock<Sender<BlockchainEvent>>>,
    vm: VM,
    runtime: Arc<Runtime>,
}

pub struct BlockchainArgs {
    pub app_prefix: String,
    pub tx_pool_sync_interval: Option<u64>,
}

impl Blockchain {
    pub async fn init(
        blockchain_args: BlockchainArgs,
    ) -> Result<Blockchain, String> {
        let BlockchainArgs {
            app_prefix,
            tx_pool_sync_interval,
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

        let vm = VM {};

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

        let blockchain = Blockchain {
            database,
            tx_pool: tx_pool.clone(),
            vm,
            bc_event_tx,
            runtime,
        };

        blockchain.insert_genesis_block().await;

        info!("Initialized Blockchain");

        Ok(blockchain)
    }

    pub async fn run(&self) {
        info!("Start running blockchain");
        match self.vm.run_vm() {
            Ok(_) => (),
            Err(err) => {
                println!("Error running vm, err: {}", err);
            }
        };

        let runtime = self.runtime.clone();
        tokio::spawn(async move {
            runtime.run().await;
        });
    }

    pub async fn insert_genesis_block(&self) {
        let genesis_block = Block {
            miner_signature: String::from("1"),
            transactions: vec![String::from("1"), String::from("2")],
            signatures: vec![String::from("1"), String::from("2")],
            created_at: String::from(""),
            height: String::from(""),
        };

        let genesis_block_hash = match genesis_block.get_hash() {
            Ok(h) => h,
            Err(_) => return,
        };

        match self.get_block(&genesis_block_hash).await {
            Ok(_) => {
                warn!("A Genesis block has already been created");
            }
            Err(_) => {
                info!("Build a genesis block");

                if let Err(_) = self.write_block(genesis_block).await {
                    warn!("Cannot create genesis block");
                };
            }
        }
    }
}
