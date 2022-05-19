use super::ledger::{self, Ledger};
use database::KeyValueDatabase;
use file_system::FS;
use logger::tinfo;
use rocksdb::Options;
use std::path::PathBuf;

pub(crate) struct Blockchain {
    pub(crate) ledger: Ledger,
}

impl Blockchain {
    pub async fn init(
        ledger_db_path: Option<String>,
    ) -> Result<Blockchain, String> {
        let ledger = Ledger::init(ledger_db_path).await?;

        let blockchain = Blockchain { ledger };

        tinfo!("saksaha", "ledger", "Initialized Blockchain");

        Ok(blockchain)
    }

    pub async fn run(&self) {
        tinfo!("saksaha", "blockchain", "Start running blockchain");

        self.ledger.write_tx();
        self.ledger.read_tx();
    }
}
