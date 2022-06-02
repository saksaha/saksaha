use super::Machine;
use crate::blockchain::{TxValue, Hash, BlockValue};

// machine api
impl Machine {
    pub(crate) async fn send_transaction(
        &self,
        tx: TxValue,
    ) -> Result<Hash, String> {
        self.blockchain.send_transaction(tx).await

        // let _ = self.storage.write_img_file();
    }

    pub(crate) async fn get_transaction(
        &self,
        tx_hash: Hash,
    ) -> Result<TxValue, String> {
        println!("blockchain get_transaction() called");
        self.blockchain.get_transaction(&tx_hash).await

        // let _ = self.storage.write_img_file();
    }

    pub(crate) async fn get_block(
        &self,
        block: &Hash,
    ) -> Result<BlockValue, String> {
        self.blockchain.get_block(block).await
    }
}
