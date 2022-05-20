use super::ledger::{self, Ledger};
use logger::tinfo;

pub(crate) struct Blockchain {
    pub(crate) ledger: Ledger,
}

pub(crate) struct TxValue<'a> {
    pub(crate) created_at: &'a str,
    pub(crate) data: &'a str,
    pub(crate) pi: &'a str,
    pub(crate) sig_vec: &'a str,
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

        self.ledger.write_tx();
        self.ledger.read_tx();
    }

    pub(crate) async fn _send_transaction<'a>(_tx_value: TxValue<'a>) {}

    pub(crate) async fn _get_transaction() {}
}
