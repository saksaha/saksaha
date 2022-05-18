use super::ledger::Ledger;
use database::KeyValueDatabase;
use logger::tinfo;

pub(crate) struct Blockchain {
    ledger: Ledger,
}

impl Blockchain {
    pub async fn init(
        ledger_db: KeyValueDatabase,
    ) -> Result<Blockchain, String> {
        let ledger = Ledger::init(ledger_db).await?;

        let blockchain = Blockchain { ledger };

        Ok(blockchain)
    }

    pub async fn run(&self) {
        tinfo!("saksaha", "blockchain", "Start running blockchain");

        self.ledger.write_tx();
        self.ledger.read_tx();
    }
}
