use crate::LedgerError;
use crate::{col_labels, LedgerCols};
use sak_kv_db::{
    BoundColumnFamily, ColumnFamilyDescriptor, DBIteratorWithThreadMode, DBWithThreadMode,
    IteratorMode, KeyValueDatabase, MultiThreaded, Options, WriteBatch, DB,
};
use sak_types::{BlockHash, Cm, MerkleRt, Sn, TxCtrOp, TxHash, TxType};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;

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
    pub(crate) async fn init(db_path: &PathBuf) -> Result<LedgerDB, LedgerError> {
        let ledger_db_path = {
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
            ColumnFamilyDescriptor::new(col_labels::TX_HASH_BY_CTR_ADDR, Options::default()),
            ColumnFamilyDescriptor::new(col_labels::TX_HASH_BY_SN, Options::default()),
            ColumnFamilyDescriptor::new(col_labels::DATA, Options::default()),
            ColumnFamilyDescriptor::new(col_labels::TX_TYPE, Options::default()),
            ColumnFamilyDescriptor::new(col_labels::CM_IDX, Options::default()),
            ColumnFamilyDescriptor::new(col_labels::CM_IDX_CM, Options::default()),
            ColumnFamilyDescriptor::new(col_labels::BLOCK_MERKLE_RT, Options::default()),
            ColumnFamilyDescriptor::new(col_labels::EMPTY_VALUE, Options::default()),
            ColumnFamilyDescriptor::new(col_labels::MERKLE_NODE, Options::default()),
            ColumnFamilyDescriptor::new(col_labels::BLOCK_HASH, Options::default()),
            ColumnFamilyDescriptor::new(col_labels::CTR_STATE, Options::default()),
            // test
            ColumnFamilyDescriptor::new(col_labels::MINT_TX_ENTITY, Options::default()),
            ColumnFamilyDescriptor::new(col_labels::POUR_TX_ENTITY, Options::default()),
            ColumnFamilyDescriptor::new(col_labels::BLOCK_ENTITY, Options::default()),
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

    pub fn put<T: Serialize>(
        &self,
        batch: &mut WriteBatch,
        column: LedgerCols,
        key: &[u8],
        value: &T,
    ) -> Result<(), LedgerError> {
        let data = serde_json::to_vec(value)?;

        let cf = self.make_cf_handle(&self.db, column.as_str())?;

        batch.put_cf(&cf, key, data);

        Ok(())
    }

    pub fn get_ser<T: Serialize + DeserializeOwned>(
        &self,
        column: LedgerCols,
        key: &[u8],
    ) -> Result<Option<T>, LedgerError> {
        let cf = self.make_cf_handle(&self.db, column.as_str())?;

        match self.db.get_cf(&cf, key)? {
            Some(ref v) => {
                let arr = serde_json::from_slice(v)?;

                Ok(Some(arr))
            }
            None => Ok(None),
        }
    }

    pub fn iter(
        &self,
        column: LedgerCols,
    ) -> Result<DBIteratorWithThreadMode<DBWithThreadMode<MultiThreaded>>, LedgerError> {
        let cf = self.make_cf_handle(&self.db, column.as_str())?;

        Ok(self.db.iterator_cf(&cf, IteratorMode::End))
    }
}
