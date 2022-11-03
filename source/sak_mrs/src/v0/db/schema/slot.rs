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
                    idx: (m.idx),
                };

                Ok(Some(mrs_data))
            }
            _ => Err(format!("Some data are missing, mrs_key: {}", mrs_key).into()),
        }
    }

    pub async fn put_data(&self, mrs_entity: MrsEntity) -> Result<String, MRSError> {
        let mut batch = WriteBatch::default();

        self.put_ser(
            &mut batch,
            CFSenum::MrsEntity,
            mrs_entity.mrs_key.as_bytes(),
            &mrs_entity,
        )?;

        self.db.write(batch)?;

        Ok(mrs_entity.mrs_key)
    }
}
