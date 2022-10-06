use crate::v0::db::MRSDB;
use crate::MRSError;
use colored::Colorize;
use sak_crypto::hasher::MiMC;
use sak_crypto::MerkleTree;
use sak_logger::info;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::broadcast;

pub struct MRS {
    db: MRSDB,
}

pub struct MRSArgs {
    pub mrs_db_path: PathBuf,
}

impl MRS {
    pub async fn init(mrs_args: MRSArgs) -> Result<MRS, MRSError> {
        let MRSArgs { mrs_db_path } = mrs_args;

        let db = MRSDB::init(&mrs_db_path).await?;

        let hasher = MiMC::new();

        let merkle_tree = MerkleTree::new(32 as u32);

        let mrs = MRS { db };

        info!("Initialized Mutable record storage (MRS)",);

        Ok(mrs)
    }

    pub async fn run(&self) {}
}
