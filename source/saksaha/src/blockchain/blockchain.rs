use std::sync::Arc;

use super::ledger::{Hashing, Ledger};
use logger::tinfo;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

pub(crate) struct Blockchain {
    pub(crate) ledger: Ledger,
    pub(crate) transactions: Arc<RwLock<Vec<Hash>>>,
}

pub(crate) struct BlockchainArgs {
    pub(crate) app_prefix: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct TxValue {
    pub(crate) created_at: String,
    pub(crate) data: String,
    pub(crate) pi: String,
    pub(crate) sig_vec: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Hash {
    pub hash: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct BlockValue {
    pub(crate) tx_pool: Vec<String>,
    pub(crate) sig_vec: Vec<String>,
    pub(crate) created_at: String,
    pub(crate) height: String,
}

impl Blockchain {
    pub(crate) async fn init(
        blockchain_args: BlockchainArgs,
        // db_prefix: Option<String>,
    ) -> Result<Blockchain, String> {
        let BlockchainArgs { app_prefix } = blockchain_args;

        let ledger = Ledger::init(&app_prefix).await?;

        let blockchain = Blockchain {
            ledger,
            transactions: Arc::new(RwLock::new(vec![])),
        };

        tinfo!("saksaha", "ledger", "Initialized Blockchain");

        Ok(blockchain)
    }

    pub(crate) async fn run(&self) {
        tinfo!("saksaha", "blockchain", "Start running blockchain");
    }

    pub(crate) async fn send_transaction(
        &self,
        tx_value: TxValue,
    ) -> Result<Hash, String> {
        let mut transactions_guard = self.transactions.write().await;
        transactions_guard.push(tx_value.get_hash()?);
        self.ledger.write_tx(tx_value).await
    }

    pub(crate) async fn get_transaction(
        &self,
        tx_hash: &Hash,
    ) -> Result<TxValue, String> {
        self.ledger.read_tx(tx_hash).await
    }

    pub(crate) async fn get_block(
        &self,
        block: &Hash,
    ) -> Result<BlockValue, String> {
        self.ledger.get_block(block).await
    }
}
