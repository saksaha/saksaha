use crate::v0::db::MRSDB;
use crate::MRSError;
use colored::Colorize;
use sak_crypto::hasher::MiMC;
use sak_crypto::MerkleTree;
use sak_logger::info;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::broadcast;

pub struct SakMRS {
    db: MRSDB,
    // ctr_addr: String,
}

impl SakMRS {
    pub async fn init<P: AsRef<Path>>(mrs_db_path: P) -> Result<Self, MRSError> {
        let db = MRSDB::init(&mrs_db_path).await?;

        let hasher = MiMC::new();

        let merkle_tree = MerkleTree::new(32 as u32);

        let mrs = SakMRS { db };

        info!("Initialized Mutable record storage (MRS)",);

        Ok(mrs)
    }

    pub async fn run(&self) {}
}
