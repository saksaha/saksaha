use crate::v0::db::MRSDB;
use crate::MRSError;
use colored::Colorize;
use sak_crypto::hasher::MiMC;
use sak_crypto::MerkleTree;
use sak_kv_db::WriteBatch;
use sak_logger::info;
use sak_store_interface::MRSInterface;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::{collections::HashMap, convert::TryInto};
use tokio::sync::broadcast;

pub struct SakMRS {
    db: MRSDB,
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

        let db = MRSDB::init(&mrs_db_path).await?;

        // let hasher = MiMC::new();

        // let merkle_tree = MerkleTree::new(32 as u32);

        let mrs = SakMRS { db };

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

impl MRSInterface for SakMRS {
    fn get_mrs_data(&self, key: &String) -> Result<Option<String>, MRSError> {
        self.db.get_dummy(key)
    }
    fn put_mrs_data(&self, key: &String, value: &String) -> Result<(), MRSError> {
        let mut batch = WriteBatch::default();

        self.db.batch_put_dummy(&mut batch, key, value);

        self.db.db.write(batch)?;

        Ok(())
    }
}
