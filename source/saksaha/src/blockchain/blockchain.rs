use super::ledger::{self, Ledger};
use logger::tinfo;
use serde::{Deserialize, Serialize};

pub(crate) struct Blockchain {
    pub(crate) ledger: Ledger,
}

pub(crate) struct BlockchainArgs {
    pub(crate) app_prefix: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct TxValue {
    pub(crate) created_at: String,
    pub(crate) data: String,
    pub(crate) pi: String,
    pub(crate) sig_vec: String,
}

impl Blockchain {
    pub(crate) async fn init(
        blockchain_args: BlockchainArgs,
        // db_prefix: Option<String>,
    ) -> Result<Blockchain, String> {
        let BlockchainArgs { app_prefix } = blockchain_args;

        let ledger = Ledger::init(&app_prefix).await?;

        let blockchain = Blockchain { ledger };

        tinfo!("saksaha", "ledger", "Initialized Blockchain");

        Ok(blockchain)
    }

    pub(crate) async fn run(&self) {
        tinfo!("saksaha", "blockchain", "Start running blockchain");
    }

    pub(crate) async fn send_transaction<'a>(
        &self,
        tx_value: TxValue,
    ) -> Result<String, String> {
        match self.ledger.write_tx(tx_value).await {
            Ok(_) => return Ok(format!("Successfully write a tx",)),
            Err(err) => {
                return Err(format!(
                    "Error initializing key value database, err: {}",
                    err
                ));
            }
        }
    }

    pub(crate) async fn get_transaction() {
        // TODO need to implement
    }
}
