use super::ledger::{self, Ledger};
use logger::tinfo;

pub(crate) struct Blockchain {
    pub(crate) ledger: Ledger,
}

pub(crate) struct TxValue {
    pub(crate) created_at: &'static str,
    pub(crate) data: &'static str,
    pub(crate) pi: &'static str,
    pub(crate) sig_vec: &'static str,
}

impl Blockchain {
    pub(crate) async fn init(
        ledger_db_path: Option<String>,
    ) -> Result<Blockchain, String> {
        let ledger = Ledger::init(ledger_db_path).await?;

        let blockchain = Blockchain { ledger };

        tinfo!("saksaha", "ledger", "Initialized Blockchain");

        Ok(blockchain)
    }

    pub(crate) async fn run(&self) {
        tinfo!("saksaha", "blockchain", "Start running blockchain");

        // self.ledger.write_tx();
        // self.ledger.read_tx();
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

    pub(crate) async fn get_transaction() {}
}
