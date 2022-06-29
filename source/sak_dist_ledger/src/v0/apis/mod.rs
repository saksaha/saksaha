mod ctr;

use crate::{Consensus, DistLedger, LedgerError, StateUpdate};
use log::warn;
use sak_contract_std::{Request, Storage};
use sak_types::{Block, BlockCandidate, Tx};
use sak_vm::CtrFn;
use std::{collections::HashMap, sync::Arc};

impl DistLedger {
    pub async fn get_blocks(
        &self,
        block_hashes: Vec<&String>,
    ) -> Result<Vec<Block>, LedgerError> {
        self.ledger_db.get_blocks(block_hashes).await
    }

    pub async fn get_latest_block_hash(
        &self,
    ) -> Result<Option<(String, String)>, LedgerError> {
        let last_block_height =
            match self.ledger_db.get_last_block_height().await? {
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

    pub async fn tx_pool_contains(&self, tx_hash: &String) -> bool {
        self.sync_pool.contains_tx(tx_hash).await
    }

    // rpc
    pub async fn send_tx(&self, tx: Tx) -> Result<(), String> {
        self.is_valid_tx(&tx);

        self.sync_pool.insert_tx(tx).await
    }

    // peer_node
    pub async fn insert_into_pool(&self, txs: Vec<Tx>) {
        for tx in txs.into_iter() {
            if let Err(err) = self.sync_pool.insert_tx(tx).await {
                warn!("Tx pool insertion aborted, reason: {}", err);
            };
        }
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
        block_height: &String,
    ) -> Result<Option<Block>, LedgerError> {
        if let Some(block_hash) =
            self.ledger_db.get_block_hash_by_height(block_height)?
        {
            return self.ledger_db.get_block(&block_hash);
        } else {
            return Ok(None);
        }
    }

    pub async fn get_last_block_height(
        &self,
    ) -> Result<Option<String>, String> {
        self.ledger_db.get_last_block_height().await
    }

    pub async fn write_block(
        &self,
        bc: Option<BlockCandidate>,
    ) -> Result<String, LedgerError> {
        let bc = match bc {
            Some(bc) => bc,
            None => self.prepare_to_write_block().await?,
        };

        let (block, txs) = bc.extract();

        let mut state_updates = vec![];

        for tx in txs.iter() {
            if tx.has_ctr_addr() {
                let data = tx.get_data();

                if let Ok(_request) = Request::parse(data) {
                    // TODO
                    // Should be able to exec ctr
                } else {
                    if let Ok(state) = self.vm.invoke(data, CtrFn::Init) {
                        state_updates.push(StateUpdate {
                            ctr_addr: tx.get_ctr_addr().to_string(),
                            new_state: state,
                        });
                    }
                }
            }
        }

        let block_hash = match self
            .ledger_db
            .write_block(&block, &txs, state_updates)
            .await
        {
            Ok(h) => h,
            Err(err) => {
                return Err(err);
            }
        };

        if let Err(err) = self.sync_pool.insert_block(&block).await {
            warn!("Error inserting block into the sync pool, err: {}", err);
        }

        Ok(block_hash)
    }

    pub fn delete_tx(&self, key: &String) -> Result<(), LedgerError> {
        self.ledger_db.delete_tx(key)
    }

    pub async fn get_tx_pool_diff(
        &self,
        tx_hashes: Vec<String>,
    ) -> Vec<String> {
        self.sync_pool.get_tx_pool_diff(tx_hashes).await
    }

    pub async fn get_txs_from_pool(&self, tx_hashes: Vec<String>) -> Vec<Tx> {
        self.sync_pool.get_txs(tx_hashes).await
    }

    pub async fn get_ctr_state(
        &self,
        contract_addr: &String,
    ) -> Result<Option<Storage>, LedgerError> {
        self.ledger_db.get_ctr_state(contract_addr)
    }

    async fn prepare_to_write_block(
        &self,
    ) -> Result<BlockCandidate, LedgerError> {
        let txs = self.sync_pool.get_all_txs().await?;

        let bc = self.consensus.do_consensus(self, txs).await?;

        self.sync_pool.remove_txs(&bc.transactions).await?;

        Ok(bc)
    }

    pub fn is_valid_tx(&self, _tx: &Tx) -> bool {
        // TODO
        true
    }
}
