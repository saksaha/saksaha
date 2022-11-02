use crate::{
    v0::db::MRSDB,
    v0::db::{CFSenum, MrsEntity},
    MRSError,
};

use sak_kv_db::WriteBatch;
use sak_types::{Block, BlockHash, BlockHeight, Tx};

impl MRSDB {
    pub async fn put_data(
        &self,
        mrs_key: String,
        mrs_value: String,
        ib: Vec<u8>,
        timestamp: String,
        idx: u32,
    ) -> Result<String, MRSError> {
        let mut batch = WriteBatch::default();

        // let block_hash = block.get_block_hash();

        let mrs_entity = MrsEntity {
            mrs_key,
            mrs_value,
            ib,
            timestamp,
            idx,
        };

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
