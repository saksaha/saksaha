use super::ledger::Ledger;
use super::tx_pool::TxPool;
use super::BlockchainEvent;
use crate::BoxedError;
use crate::Runtime;
use log::{info, warn};
use sak_types::{Block, Transaction};
use sak_vm::VM;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::sync::{
    mpsc,
    mpsc::{Receiver, Sender},
};

const BLOCKCHAIN_EVENT_QUEUE_CAPACITY: usize = 32;

pub struct Blockchain {
    pub ledger: Ledger,
    pub bc_event_rx: RwLock<Receiver<BlockchainEvent>>,
    tx_pool: Arc<TxPool>,
    vm: VM,
    bc_event_tx: Arc<Sender<BlockchainEvent>>,
    runtime: Arc<Runtime>,
}

pub struct BlockchainArgs {
    pub app_prefix: String,
}

impl Blockchain {
    pub async fn init(
        blockchain_args: BlockchainArgs,
    ) -> Result<Blockchain, String> {
        let BlockchainArgs { app_prefix } = blockchain_args;

        let ledger = Ledger::init(&app_prefix).await?;

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
            let r = Runtime::init(tx_pool.clone(), bc_event_tx.clone());

            Arc::new(r)
        };

        let blockchain = Blockchain {
            ledger,
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

    pub async fn query_contract(&self) -> Result<&[u8], String> {
        Ok(&[])
    }

    pub async fn execute_contract(&self) -> Result<&[u8], String> {
        Ok(&[])
    }

    // rpc
    pub async fn send_transaction(
        &self,
        tx: Transaction,
    ) -> Result<(), String> {
        self.tx_pool.insert(tx).await
    }

    pub async fn write_block(&self) -> Result<&[u8], String> {
        Ok(&[])
    }

    pub async fn get_transaction(
        &self,
        tx_hash: &String,
    ) -> Result<Transaction, String> {
        self.ledger.read_tx(tx_hash).await
    }

    pub async fn get_block(
        &self,
        block_hash: &String,
    ) -> Result<Block, String> {
        self.ledger.get_block(block_hash).await
    }

    pub async fn insert_into_pool(&self, txs: Vec<Transaction>) {
        for tx in txs.into_iter() {
            if let Err(err) = self.tx_pool.insert(tx).await {
                warn!("Error inserting {}", err);
            };
        }
    }

    pub async fn compare_with_pool(
        &self,
        tx_hashes: Vec<String>,
    ) -> Vec<String> {
        self.tx_pool.get_hash_diff(tx_hashes).await
    }

    pub async fn get_ack_txs_from_pool(
        &self,
        tx_hashes: Vec<String>,
    ) -> Vec<Transaction> {
        self.tx_pool.get_ack_txs(tx_hashes).await
    }
}
