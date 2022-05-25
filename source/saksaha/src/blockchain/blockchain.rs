use super::ledger::{self, Ledger};
use logger::tinfo;
use serde::{Deserialize, Serialize};

pub(crate) struct Blockchain {
    pub(crate) ledger: Ledger,
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

impl TxValue {
    pub(crate) fn empty() -> TxValue {
        TxValue {
            created_at: String::from(""),
            data: String::from(""),
            pi: String::from(""),
            sig_vec: String::from(""),
        }
    }
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

    pub(crate) async fn send_transaction(
        &self,
        tx_value: TxValue,
    ) -> Result<String, String> {
        self.ledger.write_tx(tx_value).await
    }

    pub(crate) async fn get_transaction(
        &self,
        tx_hash: &String,
    ) -> Result<TxValue, String> {
        self.ledger.read_tx(tx_hash).await
    }
}
