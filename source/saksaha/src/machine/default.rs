use super::Machine;
use sak_types::{Block, Tx};

impl Machine {
    pub(crate) async fn send_transaction(&self, tx: Tx) -> Result<(), String> {
        self.blockchain.dist_ledger.send_tx(tx).await
    }

    pub(crate) async fn get_transaction(
        &self,
        tx_hash: String,
    ) -> Result<Tx, String> {
        println!("blockchain get_transaction() called");
        self.blockchain.dist_ledger.get_tx(&tx_hash).await
    }

    pub(crate) async fn get_block(
        &self,
        block: &String,
    ) -> Result<Block, String> {
        self.blockchain.dist_ledger.get_block(block).await
    }
}
