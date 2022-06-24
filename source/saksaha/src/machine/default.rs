use super::Machine;
use sak_types::{Block, Transaction};

impl Machine {
    pub(crate) async fn send_transaction(
        &self,
        tx: Transaction,
    ) -> Result<(), String> {
        self.blockchain.dist_ledger.send_transaction(tx).await
    }

    pub(crate) async fn get_transaction(
        &self,
        tx_hash: String,
    ) -> Result<Transaction, String> {
        println!("blockchain get_transaction() called");
        self.blockchain.dist_ledger.get_transaction(&tx_hash).await
    }

    pub(crate) async fn get_block(
        &self,
        block: &String,
    ) -> Result<Block, String> {
        self.blockchain.dist_ledger.get_block(block).await
    }
}
