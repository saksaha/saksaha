use super::{ledger::Ledger, Block, Hash, Hashable, Transaction};
use crate::blockchain::vm::VM;
use log::info;
use std::sync::Arc;
use tokio::sync::RwLock;

pub(crate) struct Blockchain {
    pub(crate) ledger: Ledger,
    pub(crate) transactions: Arc<RwLock<Vec<Hash>>>,
    pub(crate) vm: VM,
}

pub(crate) struct BlockchainArgs {
    pub(crate) app_prefix: String,
}

impl Blockchain {
    pub(crate) async fn init(
        blockchain_args: BlockchainArgs,
        // db_prefix: Option<String>,
    ) -> Result<Blockchain, String> {
        let BlockchainArgs { app_prefix } = blockchain_args;

        let ledger = Ledger::init(&app_prefix).await?;

        let vm = VM {};

        let blockchain = Blockchain {
            ledger,
            vm,
            transactions: Arc::new(RwLock::new(vec![])),
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
    ) -> Result<Hash, String> {
        let mut transactions_guard = self.transactions.write().await;
        transactions_guard.push(tx.get_hash()?);
        self.ledger.write_tx(tx).await
    }

    pub(crate) async fn get_transaction(
        &self,
        tx_hash: &Hash,
    ) -> Result<Transaction, String> {
        self.ledger.read_tx(tx_hash).await
    }

    pub(crate) async fn get_block(
        &self,
        block_hash: &Hash,
    ) -> Result<Block, String> {
        self.ledger.get_block(block_hash).await
    }
}
