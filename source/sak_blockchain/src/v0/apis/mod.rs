use crate::Blockchain;
use log::warn;
use sak_types::{Block, Transaction};

impl Blockchain {
    pub async fn query_contract(&self) -> Result<&[u8], String> {
        Ok(&[])
    }

    pub async fn execute_contract(&self) -> Result<&[u8], String> {
        Ok(&[])
    }

    pub async fn contains_in_tx_pool(&self, tx_hash: &String) -> bool {
        self.tx_pool.contains(tx_hash).await
    }

    // rpc
    pub async fn send_transaction(
        &self,
        tx: Transaction,
    ) -> Result<(), String> {
        self.tx_pool.insert(tx).await
    }

    // peer_node
    pub async fn insert_into_pool(&self, txs: Vec<Transaction>) {
        for tx in txs.into_iter() {
            if let Err(err) = self.tx_pool.insert(tx).await {
                warn!("Error inserting {}", err);
            };
        }
    }

    pub async fn get_transaction(
        &self,
        tx_hash: &String,
    ) -> Result<Transaction, String> {
        self.database.read_tx(tx_hash).await
    }

    pub async fn get_block(
        &self,
        block_hash: &String,
    ) -> Result<Block, String> {
        self.database.get_block(block_hash).await
    }

    pub async fn get_block_by_height(
        &self,
        block_height: String,
    ) -> Result<Block, String> {
        let block_hash =
            self.database.get_block_hash_by_height(block_height).await?;

        self.database.get_block(&block_hash).await
    }

    pub async fn write_block(&self, block: Block) -> Result<String, String> {
        self.database.write_block(block).await
    }

    pub fn delete_tx(&self, key: &String) -> Result<(), String> {
        self.database.delete_tx(key)
    }

    pub async fn write_tx(&self, tx: &Transaction) -> Result<String, String> {
        self.database.write_tx(tx).await
    }

    pub async fn get_tx_pool_diff(
        &self,
        tx_hashes: Vec<String>,
    ) -> Vec<String> {
        self.tx_pool.get_tx_pool_diff(tx_hashes).await
    }

    pub async fn get_ack_txs_from_pool(
        &self,
        tx_hashes: Vec<String>,
    ) -> Vec<Transaction> {
        self.tx_pool.get_ack_txs(tx_hashes).await
    }
}
