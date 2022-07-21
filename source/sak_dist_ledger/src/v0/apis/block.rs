use crate::{DistLedgerApis, LedgerError};
use sak_contract_std::Storage;
use sak_types::{Block, CtrAddr, Tx, TxCandidate};

impl DistLedgerApis {
    pub async fn get_blocks(
        &self,
        block_hashes: Vec<&String>,
    ) -> Result<Vec<Block>, LedgerError> {
        self.ledger_db.schema.get_blocks(block_hashes).await
    }

    pub async fn get_txs(
        &self,
        tx_hashes: &Vec<String>,
    ) -> Result<Vec<Tx>, LedgerError> {
        self.ledger_db.schema.get_txs(tx_hashes).await
    }

    pub async fn get_merkle_node(
        &self,
        location: &String,
    ) -> Result<Option<[u8; 32]>, LedgerError> {
        self.ledger_db.schema.get_merkle_node(location)
    }

    pub async fn get_latest_block_hash(
        &self,
    ) -> Result<Option<(u128, String)>, LedgerError> {
        let last_block_height =
            match self.ledger_db.schema.get_latest_block_height()? {
                Some(h) => h,
                None => return Ok(None),
            };

        let latest_block_hash = match self
            .ledger_db
            .schema
            .get_block_hash_by_block_height(&last_block_height)?
        {
            Some(block_hash) => block_hash.to_string(),
            None => return Ok(None),
        };

        Ok(Some((last_block_height, latest_block_hash)))
    }

    // rpc
    pub async fn send_tx(
        &self,
        tx_candidate: TxCandidate,
    ) -> Result<(), String> {
        self.sync_pool.insert_tx(tx_candidate).await
    }

    pub async fn get_tx(
        &self,
        tx_hash: &String,
    ) -> Result<Option<Tx>, LedgerError> {
        self.ledger_db.schema.get_tx(tx_hash).await
    }

    pub fn get_block(
        &self,
        block_hash: &String,
    ) -> Result<Option<Block>, LedgerError> {
        self.ledger_db.schema.get_block(block_hash)
        // self.get_block(&self.kv_db.db_instance, &self.schema, block_hash)
    }

    pub async fn get_block_by_height(
        &self,
        block_height: &u128,
    ) -> Result<Option<Block>, LedgerError> {
        if let Some(block_hash) = self
            .ledger_db
            .schema
            .get_block_hash_by_block_height(block_height)?
        {
            return self.ledger_db.schema.get_block(&block_hash);
        } else {
            return Ok(None);
        }
    }

    pub fn get_latest_block_height(&self) -> Result<Option<u128>, LedgerError> {
        self.ledger_db.schema.get_latest_block_height()
    }

    pub async fn get_ledger_cm_count(
        &self,
    ) -> Result<Option<u128>, LedgerError> {
        self.ledger_db.schema.get_ledger_cm_count()
    }

    pub async fn get_latest_tx_height(
        &self,
    ) -> Result<Option<u128>, LedgerError> {
        self.ledger_db.schema.get_latest_tx_height()
    }

    pub async fn get_latest_block_merkle_rt(
        &self,
    ) -> Result<Option<[u8; 32]>, LedgerError> {
        let latest_block_height =
            match self.ledger_db.schema.get_latest_block_height()? {
                Some(h) => h,
                None => return Ok(None),
            };

        let latest_block_hash = match self
            .ledger_db
            .schema
            .get_block_hash_by_block_height(&latest_block_height)?
        {
            Some(h) => h,
            None => {
                return Err(format!(
                    "Block hash at height ({}) does not exist",
                    latest_block_height
                )
                .into())
            }
        };

        let latest_merkle_rt = self
            .ledger_db
            .schema
            .get_block_merkle_rt(&latest_block_hash)?;

        Ok(latest_merkle_rt)
    }

    pub async fn get_ctr_state(
        &self,
        contract_addr: &CtrAddr,
    ) -> Result<Option<Storage>, LedgerError> {
        self.ledger_db.schema.get_ctr_state(contract_addr)
    }
}
