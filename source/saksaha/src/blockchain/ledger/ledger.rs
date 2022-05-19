use database::KeyValueDatabase;
use logger::tinfo;

pub(crate) struct Ledger {
    pub(crate) ledger_db: KeyValueDatabase,
}

impl Ledger {
    pub(crate) async fn init(
        ledger_db: KeyValueDatabase,
    ) -> Result<Ledger, String> {
        let ledger = Ledger { ledger_db };

        tinfo!("saksaha", "ledger", "Initializing ledger");

        Ok(ledger)
    }

    pub(crate) fn write_tx(&self) {
        let db = &self.ledger_db.db;

        db.put_cf(db.cf_handle("tx_hash").unwrap(), "4", "tx4")
            .unwrap();
    }

    pub(crate) fn read_tx(&self) {
        let db = &self.ledger_db.db;

        let val = db.get_cf(db.cf_handle("tx_hash").unwrap(), "4").unwrap();

        println!("got the tx, {:?}", val);
    }
}
