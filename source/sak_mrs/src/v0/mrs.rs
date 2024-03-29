use super::session_store::SessionStore;
use crate::v0::db::{MrsRecord, MRSDB};
use crate::MRSError;
use async_trait::async_trait;

use sak_kv_db::WriteBatch;
use sak_logger::info;
use sak_store_interface::{MRSInterface, Session};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

pub struct SakMRS {
    pub(crate) db: MRSDB,
    pub(crate) session_store: SessionStore,
}

pub struct SakMRSArgs {
    pub mrs_db_path: PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PutMrsDataArgs {
    // slot_id: Vec<u64>,
    pub data_chunk: HashMap<String, Vec<u8>>,
    pub sig: Vec<u8>,
    pub slot_id: usize,
    pub ts: usize,
    pub old_ts: usize,
}

impl SakMRS {
    pub async fn init(mrs_args: SakMRSArgs) -> Result<Self, MRSError> {
        let SakMRSArgs { mrs_db_path } = mrs_args;

        let db = MRSDB::init(&mrs_db_path)?;

        let session_store = SessionStore::init();

        let mrs = SakMRS { db, session_store };

        info!("Initialized Mutable record storage (MRS)",);

        Ok(mrs)
    }

    pub async fn run(&self) {}

    pub async fn put_data(&self, pks: Vec<usize>, args: PutMrsDataArgs) -> Result<(), MRSError> {
        // 1. old ts check
        // args.old_ts

        // 2. data chunk => accumulate

        // 3. verify_sig(accumulated data, args.ts, pk, sig) -> {0, 1}

        // 4. db. store

        Ok(())
    }
}

#[async_trait]
impl MRSInterface for SakMRS {
    fn get_mrs_data(&self, key: &String) -> Result<Option<String>, MRSError> {
        self.db.get_dummy(key)
    }

    fn put_mrs_data(&self, key: &String, value: &String) -> Result<(), MRSError> {
        let mut batch = WriteBatch::default();

        self.db.batch_put_dummy(&mut batch, key, value)?;

        self.db.db.write(batch)?;

        Ok(())
    }

    // async fn get_session(&self, session_id: String) -> Result<Session, MRSError> {
    //     let mut session_store_lock = self.session_store.lock().await;

    //     let receipt = session_store_lock
    //         .get_key_value(&session_id)
    //         .ok_or("session not found")?;
    //     session_store_lock.remove(receipt.0);
    //     let sess = *(receipt.1);
    //     Ok(sess)
    // }

    fn add_session(&self, session: Session) {
        // let mut session_store_lock = self.session_store.lock().await;
        // session_store_lock.insert(session_id, session);

        self.session_store.add_session(session);
    }
}
