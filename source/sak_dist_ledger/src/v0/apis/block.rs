use crate::{DistLedger, LedgerError};
use sak_contract_std::Storage;
use sak_types::{Block, Tx, TxCandidate};

impl DistLedger {
    pub async fn get_blocks(
        &self,
        block_hashes: Vec<&String>,
    ) -> Result<Vec<Block>, LedgerError> {
        self.ledger_db.get_blocks(block_hashes).await
    }

    pub async fn get_txs(
        &self,
        tx_hashes: &Vec<String>,
    ) -> Result<Vec<Tx>, LedgerError> {
        self.ledger_db.get_txs(tx_hashes).await
    }

    pub async fn get_merkle_node(
        &self,
        location: &String,
    ) -> Result<Option<Vec<u8>>, LedgerError> {
        self.ledger_db.get_merkle_node(location).await
    }

    pub async fn get_latest_block_hash(
        &self,
    ) -> Result<Option<(u128, String)>, LedgerError> {
        let last_block_height =
            match self.ledger_db.get_latest_block_height().await? {
                Some(h) => h,
                None => return Ok(None),
            };

        let latest_block_hash = match self
            .ledger_db
            .get_block_hash_by_height(&last_block_height)?
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
        self.ledger_db.get_tx(tx_hash).await
    }

    pub fn get_block(
        &self,
        block_hash: &String,
    ) -> Result<Option<Block>, LedgerError> {
        self.ledger_db.get_block(block_hash)
    }

    pub async fn get_block_by_height(
        &self,
        block_height: &u128,
    ) -> Result<Option<Block>, LedgerError> {
        if let Some(block_hash) =
            self.ledger_db.get_block_hash_by_height(block_height)?
        {
            return self.ledger_db.get_block(&block_hash);
        } else {
            return Ok(None);
        }
    }

    pub async fn get_latest_block_height(
        &self,
    ) -> Result<Option<u128>, LedgerError> {
        self.ledger_db.get_latest_block_height().await
    }

    pub async fn get_latest_tx_height(
        &self,
    ) -> Result<Option<u128>, LedgerError> {
        self.ledger_db.get_latest_tx_height().await
    }

    pub async fn get_latest_merkle_rt(
        &self,
    ) -> Result<Option<Vec<u8>>, LedgerError> {
        let latest_tx_height =
            match self.ledger_db.get_latest_tx_height().await? {
                Some(h) => h,
                None => {
                    return Ok(None);
                    // return Err(format!("Cannot find latest tx height").into());
                }
            };

        let latest_tx_hash = match self
            .ledger_db
            .get_tx_hash_by_height(&latest_tx_height)
            .await?
        {
            Some(h) => h,
            None => {
                return Ok(None);
            }
        };

        self.ledger_db.get_merkle_rt(&latest_tx_hash).await
    }

    pub async fn get_ctr_state(
        &self,
        contract_addr: &String,
    ) -> Result<Option<Storage>, LedgerError> {
        self.ledger_db.get_ctr_state(contract_addr)
    }
}
