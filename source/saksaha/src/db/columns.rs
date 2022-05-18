use rocksdb::{ColumnFamilyDescriptor, Options};

pub(super) mod ledger_columns {
    pub(super) const TX_HASH: &str = "tx_hash";
    pub(super) const PI: &str = "pi";
}

pub(super) fn make_ledger_cf_descriptors() -> Vec<ColumnFamilyDescriptor> {
    let columns = vec![
        (ledger_columns::TX_HASH, Options::default()),
        (ledger_columns::PI, Options::default()),
    ];

    let cf = columns
        .into_iter()
        .map(|(col_name, options)| {
            ColumnFamilyDescriptor::new(col_name, options)
        })
        .collect();

    cf
}
