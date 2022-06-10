use super::block::BlockDB;
use crate::{v0::db::tx::TxDB, BoxedError};
use log::info;

pub(crate) struct Database {
    pub(crate) tx_db: TxDB,
    pub(crate) block_db: BlockDB,
}

impl Database {
    pub async fn init(app_prefix: &String) -> Result<Database, BoxedError> {
        let tx_db = TxDB::init(&app_prefix)?;

        let block_db = BlockDB::init(&app_prefix)?;

        let database = Database { tx_db, block_db };

        info!("Initialized Database");

        Ok(database)
    }
}
