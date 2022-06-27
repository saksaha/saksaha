mod vm;

use crate::{Consensus, DistLedger, LedgerError};
use log::warn;
use sak_contract_std::{Request, Storage};
use sak_types::{Block, BlockCandidate, Tx};
use sak_vm::FnType;
use std::{collections::HashMap, sync::Arc};

impl DistLedger {
    pub async fn tx_pool_contains(&self, tx_hash: &String) -> bool {
        self.tx_pool.contains(tx_hash).await
    }

    // rpc
    pub async fn send_tx(&self, tx: Tx) -> Result<(), String> {
        self.is_valid_tx(&tx);

        self.tx_pool.insert(tx).await
    }

    // peer_node
    pub async fn insert_into_pool(&self, txs: Vec<Tx>) {
        for tx in txs.into_iter() {
            if let Err(err) = self.tx_pool.insert(tx).await {
                warn!("Error inserting {}", err);
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

    pub async fn write_block(
        &self,
        bc: Option<BlockCandidate>,
    ) -> Result<String, LedgerError> {
        let bc = match bc {
            Some(bc) => bc,
            None => self.prepare_to_write_block().await?,
        };

        let (block, txs) = bc.extract();
        // let mut states = vec![];

        for tx in txs.iter() {
            // let new_state = self.exec_ctr();
            // (key, new_state)

            if tx.is_deplying_ctr() {
                self.init_ctr(tx.get_ctr_addr(), &tx.get_data()[..4]);
            }
        }

        let block_hash = match self.ledger_db.write_block(&block, &txs).await {
            Ok(h) => h,
            Err(err) => {
                return Err(err);
            }
        };

        Ok(block_hash)
    }

    pub fn delete_tx(&self, key: &String) -> Result<(), LedgerError> {
        self.ledger_db.delete_tx(key)
    }

    pub async fn get_tx_pool_diff(
        &self,
        tx_hashes: Vec<String>,
    ) -> Vec<String> {
        self.tx_pool.get_tx_pool_diff(tx_hashes).await
    }

    pub async fn get_txs_from_pool(&self, tx_hashes: Vec<String>) -> Vec<Tx> {
        self.tx_pool.get_txs(tx_hashes).await
    }

    pub async fn set_ctr_state(
        &self,
        ctr_addr: &String,
        // field_name: &String,
        // field_value: &String,
        ctr_state: &String,
    ) -> Result<String, LedgerError> {
        self.ledger_db
            // .put_ctr_state(contract_addr, field_name, field_value)
            .put_ctr_state(ctr_addr, ctr_state)
            .await
    }

    pub async fn get_ctr_state(
        &self,
        contract_addr: &String,
        // field_name: &String,
    ) -> Result<Option<Storage>, LedgerError> {
        self.ledger_db.get_ctr_state(contract_addr)
    }

    pub async fn get_txs_from_tx_pool(&self) -> (Vec<String>, Vec<Tx>) {
        let (h, t) = self.tx_pool.get_tx_pool().await;
        (h, t)
    }

    async fn prepare_to_write_block(
        &self,
    ) -> Result<BlockCandidate, LedgerError> {
        let txs = self.tx_pool.remove_all().await?;

        let bc = self.consensus.do_consensus(self, txs).await?;

        Ok(bc)
    }

    pub fn is_valid_tx(&self, _tx: &Tx) -> bool {
        // TODO
        true
    }
}
