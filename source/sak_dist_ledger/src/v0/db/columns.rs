use sak_kv_db::{ColumnFamilyDescribable, ColumnFamilyDescriptor, Options, DB};

const TX_HASH: &str = "tx_hash";

const PI: &str = "pi";

const SIG_VEC: &str = "sig_vec";

const CREATED_AT: &str = "created_at";

const DATA: &str = "data";

const CTR_ADDR: &str = "ctr_addr";

const VALIDATOR_SIG: &str = "validator_sig";

const TX_HASHES: &str = "tx_hashes";

const WITNESS_SIGS: &str = "witness_sigs";

const BLOCK_HEIGHT: &str = "block_height";

const BLOCK_HASH: &str = "block_hash";

const CTR_STATE: &str = "ctr_state";

// pub(crate) struct

pub(crate) struct LedgerDBColumnFamily {
    // tx_hash: ColumnFamilyDescriptor,
    // pi: ColumnFamilyDescriptor,
    // sig_vec: ColumnFamilyDescriptor,
    // created_at: ColumnFamilyDescriptor,
    // data: ColumnFamilyDescriptor,
    // ctr_addr: ColumnFamilyDescriptor,
    // validator_sig: ColumnFamilyDescriptor,
    // tx_hashes: ColumnFamilyDescriptor,
    // witness_sigs: ColumnFamilyDescriptor,
    // block_height: ColumnFamilyDescriptor,
    // block_hash: ColumnFamilyDescriptor,
    // ctr_state: ColumnFamilyDescriptor,
}

impl LedgerDBColumnFamily {
    pub fn new(db: &DB) -> LedgerDBColumnFamily {
        LedgerDBColumnFamily {
            // tx_hash: make_tx_hash_cf(),
            // pi: make_pi_cf(),
            // sig_vec: make_sig_vec_cf(),
            // created_at: make_created_at_cf(),
            // data: make_data_cf(),
            // ctr_addr: make_ctr_addr_cf(),
            // validator_sig: make_validator_sig_cf(),
            // tx_hashes: make_tx_hashes_cf(),
            // witness_sigs: make_witness_sigs_cf(),
            // block_height: make_block_height_cf(),
            // block_hash: make_block_hash_cf(),
            // ctr_state: make_ctr_state_cf(),
        }
    }
}

fn make_tx_hash_cf() -> ColumnFamilyDescriptor {
    ColumnFamilyDescriptor::new(TX_HASH, Options::default())
}

fn make_pi_cf() -> ColumnFamilyDescriptor {
    ColumnFamilyDescriptor::new(PI, Options::default())
}

fn make_sig_vec_cf() -> ColumnFamilyDescriptor {
    ColumnFamilyDescriptor::new(SIG_VEC, Options::default())
}

fn make_created_at_cf() -> ColumnFamilyDescriptor {
    ColumnFamilyDescriptor::new(CREATED_AT, Options::default())
}

fn make_data_cf() -> ColumnFamilyDescriptor {
    ColumnFamilyDescriptor::new(DATA, Options::default())
}

fn make_ctr_addr_cf() -> ColumnFamilyDescriptor {
    ColumnFamilyDescriptor::new(CTR_ADDR, Options::default())
}

fn make_validator_sig_cf() -> ColumnFamilyDescriptor {
    ColumnFamilyDescriptor::new(VALIDATOR_SIG, Options::default())
}

fn make_tx_hashes_cf() -> ColumnFamilyDescriptor {
    ColumnFamilyDescriptor::new(TX_HASHES, Options::default())
}

fn make_witness_sigs_cf() -> ColumnFamilyDescriptor {
    ColumnFamilyDescriptor::new(WITNESS_SIGS, Options::default())
}

fn make_block_height_cf() -> ColumnFamilyDescriptor {
    ColumnFamilyDescriptor::new(BLOCK_HEIGHT, Options::default())
}

fn make_block_hash_cf() -> ColumnFamilyDescriptor {
    ColumnFamilyDescriptor::new(BLOCK_HASH, Options::default())
}

fn make_ctr_state_cf() -> ColumnFamilyDescriptor {
    ColumnFamilyDescriptor::new(CTR_STATE, Options::default())
}

// impl ColumnFamilyDescribable for LedgerDBColumnFamily {
//     fn get_cf_vec(&self) -> Vec<ColumnFamilyDescriptor> {
//         let mut v = vec![
//             make_tx_hash_cf(),
//             make_tx_hash_cf(),
//             make_pi_cf(),
//             make_sig_vec_cf(),
//             make_created_at_cf(),
//             make_data_cf(),
//             make_ctr_addr_cf(),
//             make_validator_sig_cf(),
//             make_tx_hashes_cf(),
//             make_witness_sigs_cf(),
//             make_block_height_cf(),
//             make_block_hash_cf(),
//             make_ctr_state_cf(),
//         ];

//         v
//     }
// }
