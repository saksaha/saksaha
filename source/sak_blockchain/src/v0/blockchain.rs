use super::vm::VM;
use super::BlockchainEvent;
use super::{ledger::Ledger, Block, Hashable, Transaction};
use log::info;
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
    pub vm: VM,
    pub bc_event_tx: Arc<Sender<BlockchainEvent>>,
    pub bc_event_rx: RwLock<Receiver<BlockchainEvent>>,
    pub tx_pool: Arc<RwLock<TxPool>>,
}

pub struct BlockchainArgs {
    pub app_prefix: String,
}

pub struct TxPool {
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
    pub async fn init(
        blockchain_args: BlockchainArgs,
    ) -> Result<Blockchain, String> {
        let BlockchainArgs { app_prefix } = blockchain_args;

        let ledger = Ledger::init(&app_prefix).await?;

        let vm = VM {};

        let blockchain = {
            let (tx, rx) = mpsc::channel(BLOCKCHAIN_EVENT_QUEUE_CAPACITY);

            Blockchain {
                ledger,
                vm,
                bc_event_tx: Arc::new(tx),
                bc_event_rx: RwLock::new(rx),
                tx_pool: Arc::new(RwLock::new(TxPool::new())),
            }
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
    }

    pub async fn send_transaction(
        &self,
        tx: Transaction,
    ) -> Result<String, String> {
        let tx_hash = tx.get_hash()?;

        // println!("{:?}\n{:?}", tx, tx_hash);

        let tx_pool = self.tx_pool.clone();
        let mut tx_pool = tx_pool.write().await;
        if let Err(err) = tx_pool.insert_transaction_to_pool(tx.clone()) {
            return Err(format!("Cannot insert to tx pool, err: {}", err,));
        };

        // Define at some other location
        if let Err(err) = self
            .bc_event_tx
            .send(BlockchainEvent::TxPoolChange(tx_hash))
            .await
        {
            return Err(format!(
                "Cannot send to tx queue, rx might have been closed, err: {}",
                err,
            ));
        }
        self.ledger.write_tx(tx).await
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
}

impl TxPool {
    pub fn insert_transaction_to_pool(
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
