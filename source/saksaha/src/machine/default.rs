use super::Machine;
use crate::blockchain::{Blockchain, TxValue};

// machine api
impl Machine {
    pub(crate) async fn send_transaction(
        &self,
        tx: TxValue,
    ) -> Result<String, String> {
        println!("blockchain send_transaction() called");
        self.blockchain.send_transaction(tx).await

        // let _ = self.storage.write_img_file();
    }

    pub(crate) async fn get_transaction(
        &self,
        tx_hash: &String,
    ) -> Result<TxValue, String> {
        println!("blockchain get_transaction() called");
        self.blockchain.get_transaction(tx_hash).await

        // let _ = self.storage.write_img_file();
    }
}
