use crate::{
    v0::db::{CFSenum, MrsRecord},
    v0::{db::MRSDB, mrs},
    MRSError,
};

use sak_kv_db::{Direction, IteratorMode, WriteBatch};
use sak_logger::warn;
use sak_types::{Block, BlockHash, BlockHeight, Tx};

impl MRSDB {
    pub fn get_record(&self, mrs_key: &String) -> Result<Option<MrsRecord>, MRSError> {
        let mrs_entity: Option<MrsRecord> = self.get(CFSenum::Record, mrs_key.as_bytes())?;

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

    pub fn get_latest_index(&self, curr_slot: &str) -> Result<Option<i32>, MRSError> {
        let curr_slot_0 = format!("{}_{}", curr_slot, 0);
        let next_slot_0 = {
            let vec_split: Vec<_> = curr_slot.split(['s'].as_ref()).collect();
            let i = vec_split[vec_split.len() - 1];
            let slot_int = (i.parse::<i32>()?) + 1;
            format!("s{}_{}", slot_int, 0)
        };

        let mut iter = self.iter_from(CFSenum::RecordKey, next_slot_0.clone())?;

        let is_curr_slot = match self.get(CFSenum::RecordKey, curr_slot_0.as_bytes())? {
            Some(k) => {
                let _: String = k;
                true
            }
            None => false,
        };

        let is_next_slot = match self.get(CFSenum::RecordKey, next_slot_0.as_bytes())? {
            Some(k) => {
                let _: String = k;
                true
            }
            None => false,
        };

        let result = if is_curr_slot {
            match iter.next() {
                Some((cm_idx, _cm)) => {
                    println!("curr_slot exists, check next_slot");
                    let slot_num = std::str::from_utf8(&cm_idx)?;

                    if is_next_slot {
                        match iter.next() {
                            Some((cm_idx, _cm)) => {
                                let slot_num = std::str::from_utf8(&cm_idx)?;
                                println!("next slot found!!!!:{}", slot_num);
                                let vec_split: Vec<_> = slot_num.split(['_'].as_ref()).collect();
                                let i = vec_split[vec_split.len() - 1];
                                let idx_int = i.parse::<i32>()? + 1;
                                println!("next_slot exists, only read or update");
                                idx_int
                            }
                            None => {
                                return Err("next slot exists, but cannot get index".into());
                            }
                        }
                    } else {
                        let vec_split: Vec<_> = slot_num.split(['_'].as_ref()).collect();
                        let i = vec_split[vec_split.len() - 1];
                        let idx_int = i.parse::<i32>()? + 1;
                        println!("next slot empty, get the latest idx");
                        idx_int
                    }
                }
                None => {
                    return Err("curr slot exists, but cannot get index".into());
                }
            }
        } else {
            println!("curr slot empty, Possibly the first index");
            0
        };

        Ok(Some(result - 1))
    }

    pub async fn put_record(&self, mrs_record: MrsRecord) -> Result<String, MRSError> {
        let mut batch = WriteBatch::default();

        let s = mrs_record.key.clone();
        let slot = s.split("_").next().unwrap_or("failed to parse Record key");
        println!("********** Put start! slot_name:{}", slot);
        let latest_idx = {
            let idx = match self.get_latest_index(slot)? {
                Some(i) => i + 1,
                None => {
                    println!("latest_idx does not exist. Possibly the first index");
                    0
                }
            };
            println!("incremented index of slot:{}\n", idx);
            format!("{}_{}", slot, idx)
        };

        self.put(
            &mut batch,
            CFSenum::Record,
            mrs_record.key.as_bytes(),
            &mrs_record,
        )?;

        self.put(
            &mut batch,
            CFSenum::RecordKey,
            latest_idx.as_bytes(),
            &mrs_record.key,
        )?;

        self.db.write(batch)?;

        Ok(mrs_record.key)
    }
}
