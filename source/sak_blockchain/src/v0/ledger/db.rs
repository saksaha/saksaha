use rocksdb::ColumnFamilyDescriptor;
use rocksdb::Options;
use sak_fs::FS;
use sak_key_db::KeyValueDatabase;

pub(crate) mod tx_columns {
    pub const TX_HASH: &str = "tx_hash";
    pub const PI: &str = "pi";
    pub const SIG_VEC: &str = "sig_vec";
    pub const CREATED_AT: &str = "tx_created_at";
    pub const DATA: &str = "data";
    pub const CONTRACT: &str = "contract";
}

pub(crate) mod block_columns {
    pub const TX_POOL: &str = "tx_pool";
    pub const SIG_VEC: &str = "block_sig_vec";
    pub const CREATED_AT: &str = "block_created_at";
    pub const HEIGHT: &str = "height";
}

pub(super) fn init_ledger_db(
    app_prefix: &String,
) -> Result<KeyValueDatabase, String> {
    let ledger_db_path = {
        let app_path = FS::create_or_get_app_path(app_prefix)?;
        let db_path = { app_path.join("db").join("ledger") };

        db_path
    };

    let options = {
        let mut o = Options::default();
        o.create_missing_column_families(true);
        o.create_if_missing(true);

        o
    };

    let cf_descriptors = make_ledger_cf_descriptors();

    let ledger_db =
        match KeyValueDatabase::new(ledger_db_path, options, cf_descriptors) {
            Ok(d) => d,
            Err(err) => {
                return Err(format!(
                    "Error initializing key value database, err: {}",
                    err
                ));
            }
        };

    Ok(ledger_db)
}

fn make_ledger_cf_descriptors() -> Vec<ColumnFamilyDescriptor> {
    let columns = vec![
        (tx_columns::TX_HASH, Options::default()),
        (tx_columns::PI, Options::default()),
        (tx_columns::SIG_VEC, Options::default()),
        (tx_columns::CREATED_AT, Options::default()),
        (tx_columns::DATA, Options::default()),
        (tx_columns::CONTRACT, Options::default()),
        (block_columns::SIG_VEC, Options::default()),
        (block_columns::TX_POOL, Options::default()),
        (block_columns::CREATED_AT, Options::default()),
        (block_columns::HEIGHT, Options::default()),
    ];

    let cf = columns
        .into_iter()
        .map(|(col_name, options)| {
            ColumnFamilyDescriptor::new(col_name, options)
        })
        .collect();

    cf
}
