use super::{ledger::Ledger, Block, Hashable, Transaction};
use crate::blockchain::vm::VM;
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::sync::{
    mpsc,
    mpsc::{Receiver, Sender},
};

pub(crate) struct Blockchain {
    pub(crate) ledger: Ledger,
    pub(crate) vm: VM,
    pub(crate) transaction_tx: Sender<String>,
    pub(crate) transaction_rx: RwLock<Receiver<String>>,
    pub(crate) tx_pool: Arc<RwLock<TxPool>>,
}

pub(crate) struct BlockchainArgs {
    pub(crate) app_prefix: String,
}

pub(crate) struct TxPool {
    transactions: Vec<Transaction>,
    unique_transactions: HashMap<String, Transaction>,
}

impl TxPool {
    pub(crate) fn new() -> TxPool {
        TxPool {
            transactions: vec![],
            unique_transactions: HashMap::new(),
        }
    }
}

impl Blockchain {
    pub(crate) async fn init(
        blockchain_args: BlockchainArgs,
    ) -> Result<Blockchain, String> {
        let BlockchainArgs { app_prefix } = blockchain_args;

        let ledger = Ledger::init(&app_prefix).await?;

        let vm = VM {};

        let blockchain = {
            let (transaction_tx, transaction_rx) = mpsc::channel(32);

            Blockchain {
                ledger,
                vm,
                transaction_tx,
                transaction_rx: RwLock::new(transaction_rx),
                tx_pool: Arc::new(RwLock::new(TxPool::new())),
            }
        };

        info!("Initialized Blockchain");

        Ok(blockchain)
    }

    pub(crate) async fn run(&self) {
        info!("Start running blockchain");

        self.vm.run_vm();
    }

    pub(crate) async fn send_transaction(
        &self,
        tx: Transaction,
    ) -> Result<String, String> {
        let tx_hash = tx.get_hash()?;

        let tx_pool = self.tx_pool.clone();
        let mut tx_pool = tx_pool.write().await;
        if let Err(err) = tx_pool.insert_transaction_to_pool(tx.clone()) {
            return Err(format!("Cannot insert to tx pool, err: {}", err,));
        };

        if let Err(err) = self.transaction_tx.send(tx_hash).await {
            return Err(format!(
                "Cannot send to tx queue, rx might have been closed, err: {}",
                err,
            ));
        }
        self.ledger.write_tx(tx).await
    }

    pub(crate) async fn get_transaction(
        &self,
        tx_hash: &String,
    ) -> Result<Transaction, String> {
        self.ledger.read_tx(tx_hash).await
    }

    pub(crate) async fn get_block(
        &self,
        block_hash: &String,
    ) -> Result<Block, String> {
        self.ledger.get_block(block_hash).await
    }
}

impl TxPool {
    pub(crate) fn insert_transaction_to_pool(
        &mut self,
        tx: Transaction,
    ) -> Result<(), String> {
        let tx_hash = match tx.get_hash() {
            Ok(hash) => hash,
            Err(err) => {
                return Err(format!(
                    "Cannot to get hash correctly, err: {}",
                    err
                ))
            }
        };

        if let Some(_v) = self.unique_transactions.insert(tx_hash, tx.clone()) {
            return Err(format!("Already inserted in tx pool"));
        };

        self.transactions.push(tx);

        Ok(())
    }
}
