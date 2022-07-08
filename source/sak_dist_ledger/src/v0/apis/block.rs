use crate::{get_tx_type, DistLedger, LedgerError, RtUpdate, StateUpdate};
use log::warn;
use sak_contract_std::{CtrCallType, Request, Storage};
use sak_types::{Block, BlockCandidate, Tx, TxCandidate, TxType};
use sak_vm::CtrFn;

impl DistLedger {
    // pub async fn get_block_candidates(
    //     &self,
    //     block_hashes: Vec<&String>,
    // ) -> Result<Vec<BlockCandidate>, LedgerError> {
    //     let blocks = self.ledger_db.get_blocks(block_hashes).await?;

    //     let mut block_candidates = vec![];

    //     for b in blocks {
    //         let tx_hashes = b.get_tx_hashes();

    //         let mut txs = vec![];

    //         for tx_hash in tx_hashes {
    //             let tx = self
    //                 .ledger_db
    //                 .get_tx(tx_hash)
    //                 .await?
    //                 .ok_or("tx (of block candidate) should be persisted")?;

    //             txs.push(tx)
    //         }

    //         let block_candidate = BlockCandidate {
    //             validator_sig: b.get_validator_sig().to_string(),
    //             transactions: txs,
    //             witness_sigs: b.get_witness_sigs().to_owned(),
    //             created_at: b.get_created_at().to_owned(),
    //             block_height: b.get_height().to_owned(),
    //             merkle_root: b.get_merkle_root().to_owned(),
    //         };

    //         block_candidates.push(block_candidate);
    //     }

    //     Ok(block_candidates)
    // }

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

    pub async fn tx_pool_contains(&self, tx_hash: &String) -> bool {
        self.sync_pool.contains_tx(tx_hash).await
    }

    // rpc
    pub async fn send_tx(
        &self,
        tx_candidate: TxCandidate,
    ) -> Result<(), String> {
        self.is_valid_tx(&tx_candidate);

        self.sync_pool.insert_tx(tx_candidate).await
    }

    // peer_node
    pub async fn insert_into_pool(&self, tx_candidates: Vec<TxCandidate>) {
        for tx in tx_candidates.into_iter() {
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

    pub async fn get_latest_block_height(
        &self,
    ) -> Result<Option<u128>, LedgerError> {
        self.ledger_db.get_latest_block_height().await
    }

    pub async fn get_latest_tx_height(&self) -> Result<Option<u128>, LedgerError> {
        self.ledger_db.get_latest_tx_height().await
    }

    pub async fn write_block(
        &self,
        bc: Option<BlockCandidate>,
    ) -> Result<Option<String>, LedgerError> {
        let bc = match bc {
            Some(bc) => bc,
            None => match self.make_block_candidate().await? {
                Some(bc) => bc,
                None => {
                    // debug!("No txs to write as a block, aborting");

                    return Ok(None);
                }
            },
        };

        let latest_block_height = self.get_latest_block_height().await?;
        let latest_tx_height = self.get_latest_tx_height().await?;

        // let (block, txs) = bc.upgrade(latest_block_height, latest_tx_height);
        let tcs = bc.tx_candidates;

        let mut state_updates = StateUpdate::new();
        let mut rt_updates = RtUpdate::new();

        for tc in tcs.iter() {
            let ctr_addr = tc.get_ctr_addr();
            let data = tc.get_data();
            let tx_type = get_tx_type(ctr_addr, data);

            match tx_type {
                TxType::ContractDeploy => {
                    let initial_ctr_state =
                        self.vm.invoke(data, CtrFn::Init)?;

                    state_updates.insert(ctr_addr.clone(), initial_ctr_state);
                }

                TxType::ContractCall => {
                    let req = Request::parse(data)?;

                    match req.ctr_call_type {
                        CtrCallType::Query => {
                            self.query_ctr(ctr_addr, req).await?;
                        }
                        CtrCallType::Execute => {
                            let new_state = match state_updates.get(ctr_addr) {
                                Some(previous_state) => {
                                    let previous_state: Storage =
                                        sak_contract_std::parse_storage(
                                            previous_state.as_str(),
                                        )?;

                                    let ctr_wasm = self
                                        .ledger_db
                                        .get_ctr_data_by_ctr_addr(ctr_addr)
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
                                None => self.execute_ctr(ctr_addr, req).await?,
                            };

                            state_updates
                                .insert(ctr_addr.clone(), new_state.clone());
                        }
                    };
                }
                TxType::Plain => (),
            };

            // get rt
            // let rt =
        }

        if let Err(err) = self.sync_pool.remove_tcs(&tcs).await {
            warn!("Error removing txs into the tx pool, err: {}", err);
        }

        if let Some(_b) = self.get_block(block.get_hash())? {
            return Err(format!(
                "This block is already persisted: block_hash: {}",
                block.get_hash()
            )
            .into());
        };

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

    pub async fn get_txs_from_pool(
        &self,
        tx_hashes: Vec<String>,
    ) -> Vec<TxCandidate> {
        self.sync_pool.get_txs(tx_hashes).await
    }

    pub async fn get_ctr_state(
        &self,
        contract_addr: &String,
    ) -> Result<Option<Storage>, LedgerError> {
        self.ledger_db.get_ctr_state(contract_addr)
    }

    async fn make_block_candidate(
        &self,
    ) -> Result<Option<BlockCandidate>, LedgerError> {
        let txs = self.sync_pool.get_all_txs().await?;

        if txs.is_empty() {
            return Ok(None);
        }

        let bc = self.consensus.do_consensus(self, txs).await?;

        self.sync_pool.remove_tcs(&bc.tx_candidates).await?;

        Ok(Some(bc))
    }

    pub fn is_valid_tx(&self, _tx: &TxCandidate) -> bool {
        // TODO
        true
    }

    // fn get_merkle_root(&self, bc: &BlockCandidate) -> String {
    //     let merkle_root = &mut self.merkle_tree.clone();

    //     // for tx in &bc.transactions {
    //     //     merkle_root.upgrade_node(tx.get_tx_height().to_owned());
    //     // }

    //     merkle_root.root().hash.to_string()
    // }

    fn get_auth_paths(&self, idx: u64, height: u64) -> Vec<u64> {
        let mut auth_path = vec![];

        let mut curr_idx = idx;

        for h in 0..height {
            let sibling_idx = get_sibling_idx(curr_idx);

            let v = self.get_latest_block_height()
            let sibling = self
                .nodes
                .get(h as usize)
                .unwrap()
                .get(sibling_idx as usize)
                .unwrap();

            let direction = if sibling_idx % 2 == 0 { true } else { false };

            let p = Path {
                direction,
                hash: sibling.hash.clone(),
            };

            auth_path.push(p);

            let parent_idx = get_parent_idx(curr_idx);
            curr_idx = parent_idx;
        }

        auth_path
    }
}

fn get_sibling_idx(idx: u64) -> u64 {
    if idx % 2 == 0 {
        idx + 1
    } else {
        idx - 1
    }
}
