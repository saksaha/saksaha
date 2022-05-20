use super::*;

#[cfg(test)]
mod test {
    use crate::blockchain::{ledger::ledger_columns, Blockchain};
    use chrono::{DateTime, Utc};
    use file_system::FS;
    use hex;
    use rocksdb::{DBWithThreadMode, SingleThreaded, WriteBatch};
    use sha3::{Digest, Sha3_256};

    struct TxValue<'a> {
        created_at: &'a str,
        data: &'a str,
        pi: &'a str,
        sig_vec: &'a str,
    }

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    async fn make_blockchain() -> Blockchain {
        let db_path = {
            let app_path = FS::create_or_get_app_path().unwrap();
            let db_path = app_path.join("db_ledger_test");
            let db_path = db_path.as_os_str().to_str().unwrap().to_owned();
            Some(db_path)
        };

        let blockchain = Blockchain::init(db_path)
            .await
            .expect("Blockchain should be initialized");

        blockchain
    }

    fn make_dummy_values() -> Vec<(String, TxValue<'static>)> {
        let tx = TxValue {
            pi: "0x123",
            sig_vec: "0x0000",
            created_at: "1346546123",
            data: "None",
        };

        let transaction_data = vec!["foo", "bob", "sandy", "land", "god"];

        let mut values = vec![];

        for (_idx, tx_data) in transaction_data.iter().enumerate() {
            let tx_hash = {
                let mut h = Sha3_256::new();
                h.update(tx_data);
                h.finalize()
            };

            let tx_hash_str = hex::encode(tx_hash);

            let tx_value = TxValue {
                pi: tx.pi,
                sig_vec: tx.sig_vec,
                created_at: tx.created_at,
                data: tx.data,
            };

            values.push((tx_hash_str, tx_value))
        }

        values
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_put_and_get_transaction() {
        init();

        let blockchain = make_blockchain().await;

        let db = blockchain.ledger.ledger_db.db;

        // let tx =
        //     Transaction::new("0x0000", "0x123", "0x0000", "1346546123", "None");

        let dummy_tx_values = make_dummy_values();

        dummy_tx_values.iter().for_each(|(tx_hash, tx_val)| {
            db.put_cf(
                db.cf_handle(ledger_columns::CREATED_AT).unwrap(),
                tx_hash,
                tx_val.created_at,
                //
            )
            .expect("Tx crated_at should be inserted");

            db.put_cf(
                db.cf_handle(ledger_columns::DATA).unwrap(),
                tx_hash,
                tx_val.data,
                //
            )
            .expect("Tx data should be inserted");

            db.put_cf(
                db.cf_handle(ledger_columns::PI).unwrap(),
                tx_hash,
                tx_val.pi,
                //
            )
            .expect("Tx pi should be inserted");

            db.put_cf(
                db.cf_handle(ledger_columns::SIG_VEC).unwrap(),
                tx_hash,
                tx_val.sig_vec,
                //
            )
            .expect("Tx sig_vec should be inserted");
        });

        dummy_tx_values.iter().for_each(|(tx_hash, _)| {
            let created_at = db
                .get_cf(
                    db.cf_handle(ledger_columns::CREATED_AT).unwrap(),
                    tx_hash,
                )
                .expect("created_at should be returned")
                .expect("created_at should exist");

            println!(
                "key: {}, got value: {:?}",
                ledger_columns::CREATED_AT,
                std::str::from_utf8(&created_at)
            );

            let created_at = db
                .get_cf(
                    db.cf_handle(ledger_columns::CREATED_AT).unwrap(),
                    tx_hash,
                )
                .expect("created_at should be returned")
                .expect("created_at should exist");

            println!(
                "key: {}, got value: {:?}",
                ledger_columns::CREATED_AT,
                std::str::from_utf8(&created_at)
            );
        });
    }

    // #[tokio::test(flavor = "multi_thread")]
    fn batchput_and_get_transaction(
        db: &DBWithThreadMode<SingleThreaded>,
        cf_val: &Vec<(&str, &str)>,
        hash: &str,
        timestamp: i64,
    ) {
        let mut batch = WriteBatch::default();
        let _: () = cf_val
            .clone()
            .into_iter()
            .map(|(cf, val)| {
                batch.put_cf(db.cf_handle(cf).unwrap(), hash, val);
            })
            .collect();
        batch.put_cf(
            db.cf_handle("tx_hash").unwrap(),
            timestamp.to_be_bytes(),
            hash,
        );
        db.write(batch).expect("failed to batchWrite");

        let val = db
            .get_cf(db.cf_handle("tx_hash").unwrap(), timestamp.to_be_bytes())
            .unwrap();
        println!(
            "key: {:?}, got value: {:?}",
            timestamp,
            std::str::from_utf8(&val.unwrap()).unwrap()
        );
        let _: () = cf_val
            .into_iter()
            .map(|(cf, _)| match db.get_cf(db.cf_handle(cf).unwrap(), hash) {
                Ok(v) => println!(
                    "cf: {}, key: {:?}, got value: {:?}",
                    cf,
                    hash,
                    std::str::from_utf8(&v.unwrap()).unwrap()
                ),
                Err(_) => (),
            })
            .collect();
    }

    // #[tokio::test(flavor = "multi_thread")]
    fn raw_iterator_to_first(
        db: &DBWithThreadMode<SingleThreaded>,
        cf_val: Vec<&str>,
    ) {
        let _: () = cf_val
            .into_iter()
            .map(|cf| {
                let mut iter = db.raw_iterator_cf(db.cf_handle(cf).unwrap());
                iter.seek_to_first();
                while iter.valid() {
                    println!(
                        "Saw {:?} {:?}",
                        std::str::from_utf8(iter.key().unwrap()),
                        std::str::from_utf8(iter.value().unwrap())
                    );
                    iter.next();
                }
            })
            .collect();
    }
}
