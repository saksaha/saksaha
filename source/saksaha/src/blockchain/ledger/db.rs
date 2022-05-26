use database::KeyValueDatabase;
use file_system::FS;
use rocksdb::ColumnFamilyDescriptor;
use rocksdb::Options;

pub(crate) mod ledger_columns {
    pub const TX_HASH: &str = "tx_hash";
    pub const PI: &str = "pi";
    pub const SIG_VEC: &str = "sig_vec";
    pub const CREATED_AT: &str = "created_at";
    pub const DATA: &str = "data";
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
        (ledger_columns::TX_HASH, Options::default()),
        (ledger_columns::PI, Options::default()),
        (ledger_columns::SIG_VEC, Options::default()),
        (ledger_columns::CREATED_AT, Options::default()),
        (ledger_columns::DATA, Options::default()),
    ];

    let cf = columns
        .into_iter()
        .map(|(col_name, options)| {
            ColumnFamilyDescriptor::new(col_name, options)
        })
        .collect();

    cf
}
