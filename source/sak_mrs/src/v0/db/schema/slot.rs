use crate::{
    v0::db::{CFSenum, MrsEntity},
    v0::{db::MRSDB, mrs},
    MRSError,
};

use sak_kv_db::WriteBatch;
use sak_types::{Block, BlockHash, BlockHeight, Tx};

impl MRSDB {
    pub fn get_data(&self, mrs_key: &String) -> Result<Option<MrsEntity>, MRSError> {
        let mrs_entity: Option<MrsEntity> = self.get_ser(CFSenum::MrsEntity, mrs_key.as_bytes())?;

        match mrs_entity {
            Some(m) => {
                let mrs_data = MrsEntity {
                    mrs_key: (m.mrs_key),
                    mrs_value: (m.mrs_value),
                    ib: (m.ib),
                    timestamp: (m.timestamp),
                    // idx: (m.idx),
                };

                Ok(Some(mrs_data))
            }
            _ => Err(format!("Some data are missing, mrs_key: {}", mrs_key).into()),
        }
    }

    pub fn get_latest_index(&self) -> Result<Option<u128>, MRSError> {
        let mut iter = self.iter(CFSenum::Idx)?;

        let (idx_bytes, _idx) = match iter.next() {
            Some(a) => a,
            None => return Ok(None),
        };

        let latest_idx = type_extension::convert_u8_slice_into_u128(&idx_bytes)?;

        Ok(Some(latest_idx))
    }

    pub async fn put_data(&self, mrs_entity: MrsEntity) -> Result<String, MRSError> {
        let mut batch = WriteBatch::default();

        let latest_idx = match self.get_latest_index()? {
            Some(i) => i + 1,
            None => {
                println!("latest_idx does not exist. Possibly the first index");
                0
            }
        };

        self.put_ser(
            &mut batch,
            CFSenum::MrsEntity,
            mrs_entity.mrs_key.as_bytes(),
            &mrs_entity,
        )?;

        self.put_ser(
            &mut batch,
            CFSenum::MrsKey,
            &latest_idx.to_be_bytes(),
            &mrs_entity,
        )?;

        self.db.write(batch)?;

        Ok(mrs_entity.mrs_key)
    }
}
