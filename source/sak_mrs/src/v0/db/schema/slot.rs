use crate::{
    v0::db::{CFSenum, MrsRecord},
    v0::{db::MRSDB, mrs},
    MRSError,
};

use sak_kv_db::WriteBatch;
use sak_logger::warn;

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

    pub fn get_latest_index(&self, curr_slot: u64) -> Result<Option<u64>, MRSError> {
        let curr_slot_0 = format!("{}_{}", curr_slot, 0);
        let next_slot_0 = format!("{}_{}", curr_slot + 1, 0);

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

        let mut iter = self.iter_from(CFSenum::RecordKey, next_slot_0.clone())?;

        let latest_index = if is_curr_slot {
            match iter.next() {
                Some((cm_idx, _cm)) => {
                    if is_next_slot {
                        match iter.next() {
                            Some((cm_idx, _cm)) => {
                                let slot_idx = std::str::from_utf8(&cm_idx)?;
                                let parsed_idx = {
                                    let vec_split: Vec<_> =
                                        slot_idx.split(['_'].as_ref()).collect();
                                    let i = vec_split[vec_split.len() - 1];
                                    i.parse::<u64>()?
                                };

                                parsed_idx
                            }
                            None => {
                                return Err("next slot exists, but cannot get index".into());
                            }
                        }
                    } else {
                        let slot_idx = std::str::from_utf8(&cm_idx)?;
                        let parsed_idx = {
                            let vec_split: Vec<_> = slot_idx.split(['_'].as_ref()).collect();
                            let i = vec_split[vec_split.len() - 1];
                            i.parse::<u64>()?
                        };

                        parsed_idx
                    }
                }
                None => {
                    return Err("curr slot exists, but cannot get index".into());
                }
            }
        } else {
            return Ok(None);
        };

        Ok(Some(latest_index))
    }

    pub async fn put_record(&self, mrs_record: MrsRecord) -> Result<String, MRSError> {
        let mut batch = WriteBatch::default();

        let key = mrs_record.key.clone();
        let s = key
            .split("_")
            .next()
            .unwrap_or("failed to parse Record key");
        let slot = s.parse::<u64>()?;

        let latest_idx = {
            let idx = match self.get_latest_index(slot)? {
                Some(i) => i + 1,
                None => {
                    warn!("latest_idx does not exist. Possibly the first index");
                    0
                }
            };
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
