use rocksdb::{ColumnFamilyDescriptor, Options};

pub(crate) mod ledger_columns {
    pub(crate) const TX_HASH: &str = "tx_hash";
    pub(crate) const PI: &str = "pi";
    pub(crate) const SIG_VEC: &str = "sig_vec";
    pub(crate) const CREATED_AT: &str = "created_at";
    pub(crate) const DATA: &str = "data";
}

pub(super) fn make_ledger_cf_descriptors() -> Vec<ColumnFamilyDescriptor> {
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
