use super::Machine;
use crate::blockchain::{Blockchain, TxValue};

// machine api
impl Machine {
    pub(crate) async fn send_transaction(&self) {
        let tx = TxValue {
            pi: String::from("0x123"),
            sig_vec: String::from("0x0000"),
            created_at: String::from("1346546123"),
            data: String::from("None"),
        };

        let _ = self.blockchain.send_transaction(tx).await;

        // let _ = self.storage.write_img_file();

        println!("blockchain send_transaction() called");
    }
}
