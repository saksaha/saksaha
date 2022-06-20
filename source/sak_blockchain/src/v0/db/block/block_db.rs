use super::block_columns;
use crate::BoxedError;
use log::debug;
use sak_fs::FS;
use sak_kv_db::{
    ColumnFamilyDescriptor, KeyValueDatabase, Options, WriteBatch,
};
use sak_types::Block;

pub(crate) struct BlockDB {
    pub(crate) kv_db: KeyValueDatabase,
}

impl BlockDB {
    pub fn init(app_prefix: &String) -> Result<BlockDB, BoxedError> {
        let block_db_path = {
            let app_path = FS::create_or_get_app_path(app_prefix)?;
            let db_path = { app_path.join("db").join("block") };

            db_path
        };

        let options = {
            let mut o = Options::default();
            o.create_missing_column_families(true);
            o.create_if_missing(true);

            o
        };

        let cf_descriptors = BlockDB::make_cf_descriptors();

        let kv_db =
            match KeyValueDatabase::new(block_db_path, options, cf_descriptors)
            {
                Ok(d) => d,
                Err(err) => {
                    return Err(format!(
                        "Error initializing key value database, err: {}",
                        err
                    )
                    .into());
                }
            };

        let d = BlockDB { kv_db };

        Ok(d)
    }

    fn make_cf_descriptors() -> Vec<ColumnFamilyDescriptor> {
        let columns = vec![
            (block_columns::VALIDATOR_SIG, Options::default()),
            (block_columns::TX_HASHES, Options::default()),
            (block_columns::WITNESS_SIGS, Options::default()),
            (block_columns::CREATED_AT, Options::default()),
            (block_columns::HEIGHT, Options::default()),
            (block_columns::BLOCK_HASH, Options::default()),
        ];

        let cf = columns
            .into_iter()
            .map(|(col_name, options)| {
                ColumnFamilyDescriptor::new(col_name, options)
            })
            .collect();

        cf
    }

    pub(crate) async fn get_block(
        &self,
        block_hash: &String,
    ) -> Result<Block, String> {
        let db = &self.kv_db.db_instance;

        let cf_handle = match db.cf_handle(block_columns::VALIDATOR_SIG) {
            Some(h) => h,
            None => {
                return Err(format!(
                    "Fail to open ledger colums {}",
                    block_columns::VALIDATOR_SIG
                ));
            }
        };

        let validator_sig = match db.get_cf(cf_handle, &block_hash) {
            Ok(val) => match val {
                Some(v) => match std::str::from_utf8(&v) {
                    Ok(vs) => vs.to_string(),
                    Err(err) => {
                        return Err(format!(
                            "Invalid utf8 given, err: {}",
                            err,
                        ));
                    }
                },
                None => {
                    return Err(format!(
                        "No matched value with tx_hash in {}, {}",
                        block_columns::VALIDATOR_SIG,
                        &block_hash,
                    ));
                }
            },
            Err(err) => {
                return Err(format!(
                    "Fail to get value from ledger columns, column: {}, \
                    err: {}",
                    block_columns::VALIDATOR_SIG,
                    err,
                ));
            }
        };

        let cf_handle = match db.cf_handle(block_columns::TX_HASHES) {
            Some(h) => h,
            None => {
                return Err(format!(
                    "Fail to open ledger columns {}",
                    block_columns::TX_HASHES
                ));
            }
        };

        let get_cf_handle = db.get_cf(cf_handle, &block_hash);

        let tx_hashes = match get_cf_handle.as_ref() {
            Ok(val) => match val.as_ref() {
                Some(v) => {
                    let th: Vec<String> = serde_json::from_slice(&v).unwrap();
                    th
                }
                None => {
                    return Err(format!(
                        "No matched value with tx_hash in {}, {}",
                        block_columns::TX_HASHES,
                        &block_hash,
                    ));
                }
            },
            Err(err) => {
                return Err(format!(
                    "Fail to get value from ledger columns, column: {}, \
                    err: {}",
                    block_columns::TX_HASHES,
                    err,
                ));
            }
        };

        let cf_handle = match db.cf_handle(block_columns::WITNESS_SIGS) {
            Some(h) => h,
            None => {
                return Err(format!(
                    "Fail to open ledger columns {}",
                    block_columns::WITNESS_SIGS
                ));
            }
        };

        let get_cf_handle = db.get_cf(cf_handle, &block_hash);

        let witness_sigs = match get_cf_handle.as_ref() {
            Ok(val) => match val.as_ref() {
                Some(v) => {
                    let th: Vec<String> = serde_json::from_slice(&v).unwrap();
                    th
                }
                None => {
                    return Err(format!(
                        "No matched value with tx_hash in {}, {}",
                        block_columns::WITNESS_SIGS,
                        &block_hash,
                    ));
                }
            },
            Err(err) => {
                return Err(format!(
                    "Fail to get value from ledger columns, column: {}, \
                    err: {}",
                    block_columns::WITNESS_SIGS,
                    err,
                ));
            }
        };

        let cf_handle = match db.cf_handle(block_columns::CREATED_AT) {
            Some(h) => h,
            None => {
                return Err(format!(
                    "Fail to open ledger columns {}",
                    block_columns::CREATED_AT
                ));
            }
        };

        let created_at = match db.get_cf(cf_handle, &block_hash) {
            Ok(val) => match val {
                Some(v) => match std::str::from_utf8(&v) {
                    Ok(vs) => vs.to_string(),
                    Err(err) => {
                        return Err(format!(
                            "Invalid utf8 given, err: {}",
                            err,
                        ));
                    }
                },
                None => {
                    return Err(format!(
                        "No matched value with tx_hash in {}, {}",
                        block_columns::CREATED_AT,
                        &block_hash,
                    ));
                }
            },
            Err(err) => {
                return Err(format!(
                    "Fail to get value from ledger columns, column: {}, \
                    err: {}",
                    block_columns::CREATED_AT,
                    err,
                ));
            }
        };

        let cf_handle = match db.cf_handle(block_columns::HEIGHT) {
            Some(h) => h,
            None => {
                return Err(format!(
                    "Fail to open ledger columns {}",
                    block_columns::HEIGHT
                ));
            }
        };

        let height = match db.get_cf(cf_handle, &block_hash) {
            Ok(val) => match val {
                Some(v) => match std::str::from_utf8(&v) {
                    Ok(vs) => vs.to_string(),
                    Err(err) => {
                        return Err(format!(
                            "Invalid utf8 given, err: {}",
                            err,
                        ));
                    }
                },
                None => {
                    return Err(format!(
                        "No matched value with tx_hash in {}, {}",
                        block_columns::HEIGHT,
                        &block_hash,
                    ));
                }
            },
            Err(err) => {
                return Err(format!(
                    "Fail to get value from ledger columns, column: {}, \
                    err: {}",
                    block_columns::HEIGHT,
                    err,
                ));
            }
        };

        let b = Block::new(
            validator_sig,
            tx_hashes,
            witness_sigs,
            created_at,
            height,
        );

        Ok(b)
    }

    pub(crate) async fn get_block_hash_by_height(
        &self,
        block_height: String,
    ) -> Result<String, String> {
        let db = &self.kv_db.db_instance;

        let cf_handle = match db.cf_handle(block_columns::BLOCK_HASH) {
            Some(h) => h,
            None => {
                return Err(format!(
                    "Fail to open ledger columns {}",
                    block_columns::BLOCK_HASH
                ));
            }
        };

        let block_hash = match db.get_cf(cf_handle, &block_height) {
            Ok(val) => match val {
                Some(v) => match std::str::from_utf8(&v) {
                    Ok(vs) => vs.to_string(),
                    Err(err) => {
                        return Err(format!(
                            "Invalid utf8 given, err: {}",
                            err,
                        ));
                    }
                },
                None => {
                    return Err(format!(
                        "No matched value with tx_hash in {}, {}",
                        block_columns::BLOCK_HASH,
                        block_height,
                    ));
                }
            },
            Err(err) => {
                return Err(format!(
                    "Fail to get value from ledger columns, column: {}, \
                    err: {}",
                    block_columns::BLOCK_HASH,
                    err,
                ));
            }
        };

        Ok(block_hash)
    }

    pub(crate) async fn write_block(
        &self,
        block: Block,
    ) -> Result<String, String> {
        let db = &self.kv_db.db_instance;

        let mut batch = WriteBatch::default();

        let block_hash = block.get_hash();

        debug!(
            "write_block(): created_at: {}, block_hash: {}",
            block.get_created_at(),
            block_hash,
        );

        let cf_handle = match db.cf_handle(block_columns::VALIDATOR_SIG) {
            Some(h) => h,
            None => {
                return Err(format!(
                    "Fail to open ledger columns {}",
                    block_columns::VALIDATOR_SIG
                ))
            }
        };

        batch.put_cf(cf_handle, &block_hash, block.get_validator_sig());

        let cf_handle = match db.cf_handle(block_columns::WITNESS_SIGS) {
            Some(h) => h,
            None => {
                return Err(format!(
                    "Fail to open ledger columns {}",
                    block_columns::WITNESS_SIGS
                ))
            }
        };

        let witness_sigs = match serde_json::to_string(block.get_witness_sigs())
        {
            Ok(v) => v,
            Err(err) => {
                return Err(format!(
                    "Cannot serialize {}, err: {}",
                    block_columns::WITNESS_SIGS,
                    err
                ))
            }
        };

        batch.put_cf(cf_handle, &block_hash, witness_sigs);

        let cf_handle = match db.cf_handle(block_columns::TX_HASHES) {
            Some(h) => h,
            None => {
                return Err(format!(
                    "Fail to open ledger columns {}",
                    block_columns::TX_HASHES
                ))
            }
        };

        let transactions = match serde_json::to_string(&block.get_tx_hashes()) {
            Ok(v) => v,
            Err(err) => {
                return Err(format!(
                    "Cannot serialize {}, err: {}",
                    block_columns::TX_HASHES,
                    err
                ))
            }
        };

        batch.put_cf(cf_handle, &block_hash, transactions);

        let cf_handle = match db.cf_handle(block_columns::CREATED_AT) {
            Some(h) => h,
            None => {
                return Err(format!(
                    "Fail to open ledger columns {}",
                    block_columns::CREATED_AT
                ))
            }
        };

        batch.put_cf(cf_handle, &block_hash, block.get_created_at());

        // put k : height && v : hash
        let cf_handle = match db.cf_handle(block_columns::BLOCK_HASH) {
            Some(h) => h,
            None => {
                return Err(format!(
                    "Fail to open ledger columns {}",
                    block_columns::BLOCK_HASH
                ))
            }
        };
        batch.put_cf(cf_handle, &block.get_height(), &block_hash);

        let cf_handle = match db.cf_handle(block_columns::HEIGHT) {
            Some(h) => h,
            None => {
                return Err(format!(
                    "Fail to open ledger columns {}",
                    block_columns::HEIGHT
                ))
            }
        };

        batch.put_cf(cf_handle, &block_hash, block.get_height());

        match db.write(batch) {
            Ok(_) => return Ok(block_hash.clone()),
            Err(err) => {
                return Err(format!("Fail to write on ledger db, err: {}", err))
            }
        }
    }
}
