use super::*;

#[cfg(test)]
mod test {
    use super::dummy::Transaction;
    use crate::blockchain::{ledger::ledger_columns, Blockchain};
    use chrono::{DateTime, Utc};
    use file_system::FS;
    use hex;
    use rocksdb::{DBWithThreadMode, SingleThreaded, WriteBatch};
    use sha3::{Digest, Sha3_256};

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_send_transaction() {
        init();

        let db_path = {
            let app_path = FS::create_or_get_app_path().unwrap();
            let db_path = app_path.join("db_ledger_test");
            let db_path = db_path.as_os_str().to_str().unwrap().to_owned();
            Some(db_path)
        };

        let blockchain = Blockchain::init(db_path).await.unwrap();

        let db = blockchain.ledger.ledger_db.db;

        let tx =
            Transaction::new("0x0000", "0x123", "0x0000", "1346546123", "None");

        let foo = vec!["foo", "bob", "sandy", "land", "god"];
        for (_idx, foo) in foo.iter().enumerate() {
            let mut tx_hash = Sha3_256::new();
            tx_hash.update(foo);
            let tx_hash = tx_hash.finalize();
            let hash_str = hex::encode(tx_hash);

            let cf_val = vec![
                // (ledger_columns::TX_HASH, &hash_str[..], &hash_str[..]),
                (ledger_columns::PI, tx.pi),
                (ledger_columns::SIG_VEC, tx.sig_vec),
                (ledger_columns::CREATED_AT, tx.created_at),
                (ledger_columns::DATA, tx.data),
            ];

            // put_and_get_transaction(&db, &cf_val);
            batchput_and_get_transaction(
                &db,
                &cf_val,
                &hash_str,
                Utc::now().timestamp(),
            );
        }
        let cf_val = vec![
            ledger_columns::TX_HASH,
            ledger_columns::PI,
            ledger_columns::SIG_VEC,
            ledger_columns::CREATED_AT,
            ledger_columns::DATA,
        ];
        raw_iterator_to_first(&db, cf_val);
        // println!("now: {}", )
    }

    fn put_and_get_transaction(
        db: &DBWithThreadMode<SingleThreaded>,
        cf_val: &Vec<(&str, &str)>,
        hash: &str,
        idx: usize,
    ) {
        let _: () = cf_val
            .clone()
            .into_iter()
            .map(|(cf, val)| {
                db.put_cf(db.cf_handle(cf).unwrap(), hash, val).unwrap();
            })
            .collect();

        let _: () = cf_val
            .into_iter()
            .map(|(cf, _)| match db.get_cf(db.cf_handle(cf).unwrap(), hash) {
                Ok(v) => println!(
                    "key: {:?}, got value: {:?}",
                    std::str::from_utf8(cf.as_bytes()),
                    std::str::from_utf8(&v.unwrap())
                ),
                Err(_) => (),
            })
            .collect();
    }

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
#[cfg(test)]
mod dummy {
    pub(crate) struct Transaction<'a> {
        pub tx_hash: &'a str,
        pub pi: &'a str,
        pub sig_vec: &'a str,
        pub created_at: &'a str,
        pub data: &'a str,
    }

    impl<'a> Transaction<'a> {
        pub(crate) fn new(
            tx_hash: &'a str,
            pi: &'a str,
            sig_vec: &'a str,
            created_at: &'a str,
            data: &'a str,
        ) -> Transaction<'a> {
            Transaction {
                tx_hash,
                pi,
                sig_vec,
                created_at,
                data,
            }
        }
    }
}
