use crate::MachineError;
use crate::{cfs, CFSenum};
use sak_kv_db::{
    BoundColumnFamily, ColumnFamilyDescriptor, KeyValueDatabase, Options, WriteBatch, DB,
};
use sak_types::{BlockHash, Cm, MerkleRt, Sn, TxCtrOp, TxHash, TxType};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;

// TODO This has to be dynamically decided
const APP_NAME: &'static str = "saksaha";

pub struct LedgerDB {
    pub(crate) db: DB,
}

#[derive(Serialize, Deserialize)]
pub struct MintTxEntity {
    pub tx_hash: TxHash,
    pub tx_type: TxType,
    pub cms: Vec<Cm>,
    pub cm_idxes: Vec<u128>,
    pub cm_count: u128,
    pub created_at: String,
    pub data: Vec<u8>,
    pub author_sig: String,
    pub ctr_addr: String,
    pub v: [u8; 32],
    pub k: [u8; 32],
    pub s: [u8; 32],
    pub tx_ctr_op: TxCtrOp,
}

#[derive(Serialize, Deserialize)]
pub struct PourTxEntity {
    pub tx_hash: TxHash,
    pub tx_type: TxType,
    pub cms: Vec<Cm>,
    pub cm_idxes: Vec<u128>,
    pub cm_count: u128,
    pub created_at: String,
    pub data: Vec<u8>,
    pub author_sig: String,
    pub ctr_addr: String,
    pub pi: Vec<u8>,
    pub sns: Vec<Sn>,
    pub prf_merkle_rts: Vec<MerkleRt>,
    pub tx_ctr_op: TxCtrOp,
}

#[derive(Serialize, Deserialize)]
pub struct BlockEntity {
    pub block_hash: BlockHash,
    pub validator_sig: String,
    pub witness_sigs: Vec<String>,
    pub tx_hashes: Vec<TxHash>,
    pub created_at: String,
    pub block_height: u128,
    pub merkle_rt: MerkleRt,
}

impl LedgerDB {
    pub(crate) async fn init(
        // app_prefix: &String,
        db_path: &PathBuf,
    ) -> Result<LedgerDB, MachineError> {
        let ledger_db_path = {
            // let db_path = Self::get_db_path(app_prefix)?;

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

        let kv_db =
            match KeyValueDatabase::new(ledger_db_path, options, Self::make_cf_descriptors()) {
                Ok(d) => d,
                Err(err) => {
                    return Err(
                        format!("Error initializing key value database, err: {}", err).into(),
                    );
                }
            };

        let database = LedgerDB {
            db: kv_db.db_instance,
        };

        Ok(database)
    }

    pub(crate) fn make_cf_descriptors() -> Vec<ColumnFamilyDescriptor> {
        vec![
            ColumnFamilyDescriptor::new(cfs::TX_HASH_BY_CTR_ADDR, Options::default()),
            ColumnFamilyDescriptor::new(cfs::TX_HASH_BY_SN, Options::default()),
            // ColumnFamilyDescriptor::new(cfs::PI, Options::default()),
            // ColumnFamilyDescriptor::new(cfs::AUTHOR_SIG, Options::default()),
            // ColumnFamilyDescriptor::new(cfs::TX_CREATED_AT, Options::default()),
            // ColumnFamilyDescriptor::new(cfs::BLOCK_CREATED_AT, Options::default()),
            // ColumnFamilyDescriptor::new(cfs::DATA, Options::default()),
            // ColumnFamilyDescriptor::new(cfs::CTR_ADDR, Options::default()),
            // ColumnFamilyDescriptor::new(cfs::TX_TYPE, Options::default()),
            // ColumnFamilyDescriptor::new(cfs::CM_IDX, Options::default()),
            ColumnFamilyDescriptor::new(cfs::CM_IDX_CM, Options::default()),
            // ColumnFamilyDescriptor::new(cfs::V, Options::default()),
            // ColumnFamilyDescriptor::new(cfs::K, Options::default()),
            // ColumnFamilyDescriptor::new(cfs::S, Options::default()),
            // ColumnFamilyDescriptor::new(cfs::SN, Options::default()),
            // ColumnFamilyDescriptor::new(cfs::CM, Options::default()),
            // ColumnFamilyDescriptor::new(cfs::CM_COUNT, Options::default()),
            ColumnFamilyDescriptor::new(cfs::BLOCK_MERKLE_RT, Options::default()),
            ColumnFamilyDescriptor::new(cfs::EMPTY_VALUE, Options::default()),
            // ColumnFamilyDescriptor::new(cfs::PRF_MERKLE_RT, Options::default()),
            ColumnFamilyDescriptor::new(cfs::MERKLE_NODE, Options::default()),
            // ColumnFamilyDescriptor::new(cfs::VALIDATOR_SIG, Options::default()),
            // ColumnFamilyDescriptor::new(cfs::TX_HASHES, Options::default()),
            // ColumnFamilyDescriptor::new(cfs::WITNESS_SIGS, Options::default()),
            // ColumnFamilyDescriptor::new(cfs::BLOCK_HEIGHT, Options::default()),
            ColumnFamilyDescriptor::new(cfs::BLOCK_HASH, Options::default()),
            ColumnFamilyDescriptor::new(cfs::CTR_STATE, Options::default()),
            // test
            ColumnFamilyDescriptor::new(cfs::MINT_TX_ENTITY, Options::default()),
            ColumnFamilyDescriptor::new(cfs::POUR_TX_ENTITY, Options::default()),
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
                return Err(format!("Fail to open ledger colums {}", col_name,));
            }
        };

        Ok(cf_handle)
    }

    pub fn put_ser<T: Serialize>(
        &mut self,
        batch: &mut WriteBatch,
        column: CFSenum,
        key: &[u8],
        value: &T,
    ) -> Result<(), MachineError> {
        let data = serde_json::to_vec(value)?;
        self.put(batch, column, key, &data);
        Ok(())
    }

    pub fn put(
        &mut self,
        batch: &mut WriteBatch,
        column: CFSenum,
        key: &[u8],
        value: &[u8],
    ) -> Result<(), MachineError> {
        let cf = self.make_cf_handle(&self.db, column.as_str())?;

        batch.put_cf(&cf, key.to_vec(), value.to_vec());

        Ok(())
    }

    pub fn get_ser<T: Serialize + Deserialize>(
        &self,
        column: CFSenum,
        key: &[u8],
    ) -> Result<Option<T>, MachineError> {
        let cf = self.make_cf_handle(&self.db, column.as_str())?;

        match self.db.get_cf(&cf, key)? {
            Some(v) => Ok(Some(serde_json::from_slice(&v)?)),
            None => Ok(None),
        }
    }

    pub fn get(&self, column: CFSenum, key: &[u8]) -> Result<Option<Vec<u8>>, MachineError> {
        self.db.get_cf(column, key)?
    }
}
