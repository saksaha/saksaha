use std::collections::HashMap;

use crate::{DistLedger, LedgerError};
use log::warn;
use sak_contract_std::Request;
use sak_types::{Block, BlockCandidate, Tx};
use sak_vm::FnType;

impl DistLedger {
    pub async fn query_contract(&self) -> Result<&[u8], String> {
        Ok(&[])
    }

    pub async fn execute_ctr(
        &self,
        ctr_addr: &String,
        fn_type: FnType,
        request: Request,
    ) -> Result<&[u8], LedgerError> {
        println!(
            "execute ctr!!, ctr_addr: {}, fn_type: {:?}",
            ctr_addr, fn_type
        );

        let ctr_wasm = self
            .ledger_db
            .get_ctr_data_by_ctr_addr(ctr_addr)
            .await?
            .ok_or("ctr data (wasm) should exist")?;

        let mut storage: HashMap<String, String> = HashMap::with_capacity(10);

        storage.insert(
            "validators".to_string(),
            serde_json::to_string(&vec![String::from(
                "\
            046885b904a8b8cdd17cc40078ed11421\
            4586f197a664d6aa33d4b46cc3b712afc\
            def3d4d808bc7843beaea9e1a4c5ddeea\
            47cbd27ea1af5ca13719a2f42c39167\
            ",
            )])
            .unwrap()
            .to_string(),
        );

        let ret = match self.vm.exec(ctr_wasm, fn_type, request, storage) {
            Ok(ret) => ret,
            Err(err) => return Err(err),
        };

        println!("returned!!!: {}", ret);

        Ok(&[])
    }

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
            println!("get_block_by_height, hash: {}", block_hash);

            return self.ledger_db.get_block(&block_hash);
        } else {
            return Ok(None);
        }
    }

    pub async fn write_block(
        &self,
        bc: &BlockCandidate,
    ) -> Result<String, LedgerError> {
        let (block, txs) = self.verify_block_candidate(bc)?;

        let tx_hashes = block.get_tx_hashes();

        let block_hash = match self.ledger_db.write_block(&block, &txs).await {
            Ok(h) => h,
            Err(err) => {
                return Err(err);
            }
        };

        match self.tx_pool.remove_txs(tx_hashes).await {
            Ok(_) => {}
            Err(_err) => {
                // TODO
                // self.database.remove_block(block_hash);
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
    ) -> Result<Option<Vec<u8>>, LedgerError> {
        self.ledger_db.get_ctr_state(contract_addr).await
    }

    pub async fn get_txs_from_tx_pool(&self) -> (Vec<String>, Vec<Tx>) {
        let (h, t) = self.tx_pool.get_tx_pool().await;
        (h, t)
    }

    fn verify_block_candidate<'a>(
        &self,
        bc: &'a BlockCandidate,
    ) -> Result<(Block, Vec<&'a Tx>), String> {
        // TODO verify

        Ok(bc.extract())
    }
}
