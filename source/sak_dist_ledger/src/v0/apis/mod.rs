mod ctr;

use crate::{DistLedger, LedgerError, StateUpdate};
use log::{debug, warn};
use sak_contract_std::{CtrCallType, Request, Storage};
use sak_types::{Block, BlockCandidate, Tx, TxType};
use sak_vm::CtrFn;

impl DistLedger {
    pub async fn get_block_candidates(
        &self,
        block_hashes: Vec<&String>,
    ) -> Result<Vec<BlockCandidate>, LedgerError> {
        let blocks = self.ledger_db.get_blocks(block_hashes).await?;

        let mut block_candidates = vec![];

        for b in blocks {
            let tx_hashes = b.get_tx_hashes();

            let mut txs = vec![];

            for tx_hash in tx_hashes {
                let tx = self
                    .ledger_db
                    .get_tx(tx_hash)
                    .await?
                    .ok_or("tx (of block candidate) should be persisted")?;

                txs.push(tx)
            }

            let block_candidate = BlockCandidate {
                validator_sig: b.get_validator_sig().to_string(),
                transactions: txs,
                witness_sigs: b.get_witness_sigs().to_owned(),
                created_at: b.get_created_at().to_owned(),
                height: b.get_height().to_owned(),
            };

            block_candidates.push(block_candidate);
        }

        Ok(block_candidates)
    }

    pub async fn get_blocks(
        &self,
        block_hashes: Vec<&String>,
    ) -> Result<Vec<Block>, LedgerError> {
        self.ledger_db.get_blocks(block_hashes).await
    }

    pub async fn get_latest_block_hash(
        &self,
    ) -> Result<Option<(u128, String)>, LedgerError> {
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

    pub async fn get_last_block_height(&self) -> Result<Option<u128>, String> {
        self.ledger_db.get_last_block_height().await
    }

    pub async fn write_block(
        &self,
        bc: Option<BlockCandidate>,
    ) -> Result<Option<String>, LedgerError> {
        let bc = match bc {
            Some(bc) => bc,
            None => match self.prepare_to_write_block().await? {
                Some(bc) => bc,
                None => {
                    debug!("No txs to write as a block, aborting");

                    return Ok(None);
                }
            },
        };

        let (block, txs) = bc.extract();

        if let Some(_b) = self.get_block(block.get_hash())? {
            return Err(format!(
                "This block is already persisted: block_hash: {}",
                block.get_hash()
            )
            .into());
        };

        let mut state_updates = StateUpdate::new(); // hashmap <K; ctr_addr => v ctr_state>

        for tx in txs.iter() {
            match tx.get_type() {
                TxType::ContractDeploy => {
                    println!("\n[Tx Iteration] deployed!");
                    let initial_ctr_state =
                        self.vm.invoke(tx.get_data(), CtrFn::Init)?;

                    {
                        let initial_ctr_state_result: Storage =
                            serde_json::from_str(
                                initial_ctr_state.clone().as_str(),
                            )
                            .unwrap();

                        println!(
                            "[>>] [Deploy] initial state: {:#?}",
                            initial_ctr_state_result
                        );
                    }

                    match state_updates
                        .insert(tx.get_ctr_addr().clone(), initial_ctr_state)
                    {
                        Some(_) => {
                            println!("[state_update] the state of the contract has been initialized in the same block before");
                        }
                        None => {
                            println!("[state_update] contract state has been initialized");
                        }
                    }
                }

                TxType::ContractCall => {
                    println!("\n[Tx Iteration] called!");
                    let req = Request::parse(tx.get_data()).expect(
                        "tx.data should be in form of struct `Request` to invoke contract function",
                    );

                    match req.ctr_call_type {
                        CtrCallType::Query => {
                            // self.query_ctr(tx.get_ctr_addr(), req).await?;

                            {
                                // log

                                let validator = self
                                    .query_ctr(tx.get_ctr_addr(), req)
                                    .await?;

                                println!(
                                    "[>>]   [Query] validator: {:#?}",
                                    validator
                                );
                            }
                        }
                        CtrCallType::Execute => {
                            let new_state =
                                match state_updates.get(tx.get_ctr_addr()) {
                                    Some(previous_state) => {
                                        //

                                        let previous_state: Storage =
                                        serde_json::from_str(
                                            previous_state.as_str(),
                                        )
                                        .expect(
                                            "previou state should be a storage",
                                        );
                                        let ctr_wasm = self
                                            .ledger_db
                                            .get_ctr_data_by_ctr_addr(
                                                tx.get_ctr_addr(),
                                            )
                                            .await?
                                            .ok_or(
                                                "ctr data (wasm) should exist",
                                            )?;

                                        let ctr_fn =
                                            CtrFn::Execute(req, previous_state);

                                        let ret =
                                            self.vm.invoke(ctr_wasm, ctr_fn)?;

                                        ret
                                    }
                                    None => {
                                        self.execute_ctr(tx.get_ctr_addr(), req)
                                            .await?
                                    }
                                };

                            {
                                // log

                                let new_state_result: Storage =
                                    serde_json::from_str(new_state.as_str())
                                        .unwrap();
                                println!(
                                    "[>>] [Execute] updated state: {:#?}",
                                    new_state_result
                                );
                            }

                            state_updates.insert(
                                tx.get_ctr_addr().clone(),
                                new_state.clone(),
                            );
                        }
                    };
                }
                TxType::Plain => (),
            };

            // match state {
            //     Some(state) => {
            //         state_updates.push(StateUpdate {
            //             ctr_addr: tx.get_ctr_addr().to_string(),
            //             new_state: state,
            //         });
            //     }
            //     None => {}
            // }
        }

        if let Err(err) = self.sync_pool.remove_txs(&txs).await {
            warn!("Error removing txs into the tx pool, err: {}", err);
        }

        let block_hash = match self
            .ledger_db
            .write_block(&block, &txs, &state_updates)
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

        Ok(Some(block_hash))
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
    ) -> Result<Option<BlockCandidate>, LedgerError> {
        let txs = self.sync_pool.get_all_txs().await?;

        if txs.is_empty() {
            return Ok(None);
        }

        let bc = self.consensus.do_consensus(self, txs).await?;

        self.sync_pool.remove_txs(&bc.transactions).await?;

        Ok(Some(bc))
    }

    pub fn is_valid_tx(&self, _tx: &Tx) -> bool {
        // TODO
        true
    }
}
