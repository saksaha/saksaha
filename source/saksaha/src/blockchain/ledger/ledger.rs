use super::db;
use database::KeyValueDatabase;
use logger::tinfo;

pub(crate) struct Ledger {
    pub(crate) ledger_db: KeyValueDatabase,
}

impl Ledger {
    pub(crate) async fn init(
        ledger_db_path: Option<String>,
    ) -> Result<Ledger, String> {
        let ledger_db = match db::init_ledger_db(ledger_db_path) {
            Ok(d) => d,
            Err(err) => {
                return Err(format!(
                    "Could not initialize ledger db, err: {}",
                    err
                ));
            }
        };

        let ledger = Ledger { ledger_db };

        tinfo!("saksaha", "ledger", "Initialized Ledger (and ledger db)");

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

        // println!("got the tx, {:?}", val);
    }
}
