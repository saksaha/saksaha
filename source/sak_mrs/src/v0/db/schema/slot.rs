use crate::{
    v0::db::{CFSenum, MrsRecord},
    v0::{db::MRSDB, mrs},
    MRSError,
};

use sak_kv_db::WriteBatch;
use sak_types::{Block, BlockHash, BlockHeight, Tx};

impl MRSDB {
    pub fn get_data(&self, mrs_key: &String) -> Result<Option<MrsRecord>, MRSError> {
        let mrs_entity: Option<MrsRecord> = self.get_ser(CFSenum::Record, mrs_key.as_bytes())?;

        match mrs_entity {
            Some(m) => {
                let mrs_data = MrsRecord {
                    key: (m.key),
                    value: (m.value),
                    ib: (m.ib),
                    timestamp: (m.timestamp),
                    // idx: (m.idx),
                };

                Ok(Some(mrs_data))
            }
            _ => Err(format!("Some data are missing, mrs_key: {}", mrs_key).into()),
        }
    }

    pub fn get_latest_index(&self, slot_name) -> Result<Option<u128>, MRSError> {
        let mut iter = self.iter(CFSenum::RecordKey)?;

        let (idx_bytes, _idx) = match iter.next() {
            Some(a) => a,
            None => return Ok(None),
        };

        let latest_idx = type_extension::convert_u8_slice_into_u128(&idx_bytes)?;

        Ok(Some(latest_idx))
    }

    pub async fn put_data(&self, mrs_record: MrsRecord) -> Result<String, MRSError> {
        let mut batch = WriteBatch::default();

        let slot = mrs_record.key.clone();
        let slot_name = slot
            .split("_")
            .next()
            .unwrap_or("failed to parse Record key");

        let latest_idx = {
            let latest_idx = match self.get_latest_index(slot_name)? {
                Some(i) => i + 1,
                None => {
                    println!("latest_idx does not exist. Possibly the first index");
                    0
                }
            };

            let slotnum = "slotnum_15";
            let vec_split: Vec<_> = slotnum.split(['_'].as_ref()).collect();
            let idx = {
                let i = vec_split[vec_split.len() - 1];
                let idx_int = (i.parse::<i32>()?) + 1;
                format!("{}", idx_int)
            };

            let mut s = slot_name.to_string();
            s.push_str(&idx);
            s
        };

        self.put_ser(
            &mut batch,
            CFSenum::Record,
            mrs_record.key.as_bytes(),
            &mrs_record,
        )?;

        self.put_ser(
            &mut batch,
            CFSenum::RecordKey,
            latest_idx.as_bytes(),
            &mrs_record.key,
        )?;

        self.db.write(batch)?;

        Ok(mrs_record.key)
    }
}
