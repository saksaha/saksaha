use super::db;
use crate::blockchain::blockchain::TxValue;
use database::KeyValueDatabase;
use logger::tinfo;
use rocksdb::WriteBatch;
use sha3::{Digest, Sha3_256};

pub(crate) struct Ledger {
    pub(crate) ledger_db: KeyValueDatabase,
}

impl Ledger {
    pub(crate) async fn init(
        // db_prefix: Option<String>,
        app_prefix: &String,
    ) -> Result<Ledger, String> {
        let ledger_db = match db::init_ledger_db(&app_prefix) {
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

    pub(crate) async fn write_tx(
        &self,
        tx_value: TxValue,
    ) -> Result<(), String> {
        let db = &self.ledger_db.db;

        let mut batch = WriteBatch::default();
        let tx_hash = {
            let mut h = Sha3_256::new();
            h.update(tx_value.created_at.clone());
            h.finalize()
        };

        batch.put_cf(
            db.cf_handle(db::ledger_columns::CREATED_AT)
                .expect("Fail to open ledger columns created_at"),
            tx_hash,
            tx_value.created_at,
        );

        batch.put_cf(
            db.cf_handle(db::ledger_columns::DATA)
                .expect("Fail to open ledger columns data"),
            tx_hash,
            tx_value.data,
        );

        batch.put_cf(
            db.cf_handle(db::ledger_columns::PI)
                .expect("Fail to open ledger columns pi"),
            tx_hash,
            tx_value.pi,
        );

        batch.put_cf(
            db.cf_handle(db::ledger_columns::SIG_VEC)
                .expect("Fail to open ledger columns sig_vec"),
            tx_hash,
            tx_value.sig_vec,
        );
        db.write(batch).expect("failed to batchWrite");

        Ok(())
    }

    pub(crate) fn read_tx(&self) {
        let db = &self.ledger_db.db;

        let val = db.get_cf(db.cf_handle("tx_hash").unwrap(), "4").unwrap();

        // println!("got the tx, {:?}", val);
    }
}
