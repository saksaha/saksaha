use super::tx_pool::TxPool;
use super::BlockchainEvent;
use crate::BoxedError;
use crate::Database;
use crate::Runtime;
use log::{info, warn};
use sak_task_queue::TaskQueue;
use sak_types::{Block, Transaction};
use sak_vm::VM;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::sync::{
    mpsc,
    mpsc::{Receiver, Sender},
};

const BLOCKCHAIN_EVENT_QUEUE_CAPACITY: usize = 32;

pub struct Blockchain {
    pub(crate) database: Database,
    pub bc_event_rx: RwLock<Receiver<BlockchainEvent>>,
    pub(crate) tx_pool: Arc<TxPool>,
    vm: VM,
    bc_event_tx: Arc<Sender<BlockchainEvent>>,
    pub runtime: Arc<Runtime>,
}

pub struct BlockchainArgs {
    pub app_prefix: String,
}

impl Blockchain {
    pub async fn init(
        blockchain_args: BlockchainArgs,
    ) -> Result<Blockchain, String> {
        let BlockchainArgs { app_prefix } = blockchain_args;

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

        let (bc_event_tx, bc_event_rx) = {
            let (tx, rx) = mpsc::channel(BLOCKCHAIN_EVENT_QUEUE_CAPACITY);

            (Arc::new(tx), RwLock::new(rx))
        };

        let runtime = {
            let sync_task_queue = {
                let q = TaskQueue::new(BLOCKCHAIN_EVENT_QUEUE_CAPACITY);
                Arc::new(q)
            };

            let r = Runtime::init(tx_pool.clone(), sync_task_queue, None);

            Arc::new(r)
        };

        // let apis = Apis::new(database);

        let blockchain = Blockchain {
            // apis,
            database,
            vm,
            bc_event_tx: bc_event_tx.clone(),
            bc_event_rx,
            tx_pool: tx_pool.clone(),
            runtime,
        };

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
}
