use crate::cfs;
use crate::LedgerError;
use log::info;
use sak_kv_db::{
    BoundColumnFamily, ColumnFamilyDescriptor, KeyValueDatabase, Options, DB,
};
use std::path::PathBuf;
use std::sync::Arc;

// TODO This has to be dynamically decided
const APP_NAME: &'static str = "saksaha";

pub struct LedgerDB {
    // pub(crate) schema: LedgerDBSchema,
    pub(crate) db: DB,
}

impl LedgerDB {
    pub(crate) async fn init(
        app_prefix: &String,
    ) -> Result<LedgerDB, LedgerError> {
        let ledger_db_path = {
            let db_path = Self::get_db_path(app_prefix)?;

            if !db_path.exists() {
                std::fs::create_dir_all(db_path.clone())?;
            }

            db_path
        };

        let options = {
            let mut o = Options::default();
            o.create_missing_column_families(true);
            o.create_if_missing(true);

            o
        };

        let kv_db = match KeyValueDatabase::new(
            ledger_db_path,
            options,
            Self::make_cf_descriptors(),
        ) {
            Ok(d) => d,
            Err(err) => {
                return Err(format!(
                    "Error initializing key value database, err: {}",
                    err
                )
                .into());
            }
        };

        // let schema = LedgerDBSchema::new(kv_db.db_instance);

        let database = LedgerDB {
            db: kv_db.db_instance,
        };

        info!("Initialized Database");

        Ok(database)
    }

    pub fn get_db_path(app_prefix: &str) -> Result<PathBuf, LedgerError> {
        let app_path = sak_fs::get_app_root_path(APP_NAME)?.join(app_prefix);

        let db_path = app_path.join("db").join("ledger");

        Ok(db_path)
    }

    pub(crate) fn make_cf_descriptors() -> Vec<ColumnFamilyDescriptor> {
        vec![
            // ColumnFamilyDescriptor::new(
            //     cfs::TX_HASH_BY_HEIGHT,
            //     Options::default(),
            // ),
            ColumnFamilyDescriptor::new(
                cfs::TX_HASH_BY_CTR_ADDR,
                Options::default(),
            ),
            ColumnFamilyDescriptor::new(cfs::TX_HASH_BY_SN, Options::default()),
            ColumnFamilyDescriptor::new(cfs::PI, Options::default()),
            ColumnFamilyDescriptor::new(cfs::AUTHOR_SIG, Options::default()),
            ColumnFamilyDescriptor::new(cfs::TX_CREATED_AT, Options::default()),
            ColumnFamilyDescriptor::new(
                cfs::BLOCK_CREATED_AT,
                Options::default(),
            ),
            ColumnFamilyDescriptor::new(cfs::DATA, Options::default()),
            ColumnFamilyDescriptor::new(cfs::CTR_ADDR, Options::default()),
            // ColumnFamilyDescriptor::new(cfs::TX_HEIGHT, Options::default()),
            ColumnFamilyDescriptor::new(cfs::TX_TYPE, Options::default()),
            // ColumnFamilyDescriptor::new(cfs::CM, Options::default()),
            ColumnFamilyDescriptor::new(cfs::CM_IDX, Options::default()),
            ColumnFamilyDescriptor::new(cfs::V, Options::default()),
            ColumnFamilyDescriptor::new(cfs::K, Options::default()),
            ColumnFamilyDescriptor::new(cfs::S, Options::default()),
            ColumnFamilyDescriptor::new(cfs::SN_1, Options::default()),
            ColumnFamilyDescriptor::new(cfs::SN_2, Options::default()),
            ColumnFamilyDescriptor::new(cfs::CM_1, Options::default()),
            ColumnFamilyDescriptor::new(cfs::CM_2, Options::default()),
            ColumnFamilyDescriptor::new(
                cfs::BLOCK_MERKLE_RT,
                Options::default(),
            ),
            ColumnFamilyDescriptor::new(cfs::PRF_MERKLE_RT, Options::default()),
            ColumnFamilyDescriptor::new(cfs::MERKLE_NODE, Options::default()),
            ColumnFamilyDescriptor::new(cfs::VALIDATOR_SIG, Options::default()),
            ColumnFamilyDescriptor::new(cfs::TX_HASHES, Options::default()),
            ColumnFamilyDescriptor::new(cfs::WITNESS_SIGS, Options::default()),
            ColumnFamilyDescriptor::new(cfs::BLOCK_HEIGHT, Options::default()),
            ColumnFamilyDescriptor::new(cfs::BLOCK_HASH, Options::default()),
            ColumnFamilyDescriptor::new(cfs::CTR_STATE, Options::default()),
            // ColumnFamilyDescriptor::new(
            //     cfs::BLOCK_CM_COUNT,
            //     Options::default(),
            // ),
            // ColumnFamilyDescriptor::new(
            //     cfs::LEDGER_CM_COUNT,
            //     Options::default(),
            // ),
        ]
    }

    pub(crate) fn make_cf_handle<'a>(
        &self,
        db: &'a DB,
        col_name: &'static str,
    ) -> Result<Arc<BoundColumnFamily<'a>>, String> {
        let cf_handle = match db.cf_handle(col_name) {
            Some(h) => h,
            None => {
                return Err(
                    format!("Fail to open ledger colums {}", col_name,),
                );
            }
        };

        Ok(cf_handle)
    }
}
