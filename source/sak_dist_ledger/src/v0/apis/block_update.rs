use crate::{CtrStateUpdate, DistLedgerApis, LedgerError, MerkleUpdate};
use colored::Colorize;
use log::{debug, error, info, warn};
use sak_contract_std::{CtrCallType, Request, Storage};
use sak_types::{
    Block, BlockCandidate, MintTxCandidate, PourTxCandidate, Tx, TxCandidate,
    TxCtrOp,
};
use sak_vm::CtrFn;

impl DistLedgerApis {
    pub async fn insert_genesis_block(
        &self,
        genesis_block: BlockCandidate,
    ) -> Result<String, String> {
        let persisted_gen_block_hash = if let Some(b) =
            match self.get_block_by_height(&0).await {
                Ok(b) => b,
                Err(err) => return Err(err.to_string()),
            } {
            let block_hash = b.get_block_hash().to_string();

            info!(
                "Genesis block is already persisted, block_hash: {}",
                block_hash.green(),
            );

            block_hash
        } else {
            info!("Genesis block not found, writing");

            let b = match self.write_block(Some(genesis_block)).await {
                Ok(b) => b.ok_or(
                    "genesis block should have been written as it \
                        does not exist at the moment",
                )?,
                Err(err) => return Err(err.to_string()),
            };

            b
        };

        Ok(persisted_gen_block_hash.to_string())
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

        let next_block_height = match self.get_latest_block_height()? {
            Some(h) => h + 1,
            None => {
                warn!("Block height does not exist. Possibly the first block");
                0
            }
        };

        let ledger_cm_count = match self.get_ledger_cm_count().await? {
            Some(h) => h,
            None => {
                warn!(
                    "Total cm count does not exist. Possibly the first block"
                );

                0
            }
        };

        let next_tx_height = match self.get_latest_tx_height().await? {
            Some(th) => th + 1,
            None => 0,
        };

        let tcs = &bc.tx_candidates;
        let mut ctr_state_update = CtrStateUpdate::new();
        let mut merkle_update = MerkleUpdate::new();

        debug!(
            "write_block, tc count: {}, next_block_height: {}, \
            next_tx_height: {}, ledger_cm_count: {}",
            tcs.len(),
            next_block_height,
            next_tx_height,
            ledger_cm_count,
        );

        let mut added_cm_count: u128 = 0;
        for tx_candidate in tcs {
            let cm_count = match tx_candidate {
                TxCandidate::Mint(tc) => {
                    handle_mint_tx_candidate(
                        self,
                        tc,
                        &mut ctr_state_update,
                        &mut merkle_update,
                        added_cm_count + ledger_cm_count,
                    )
                    .await?
                }
                TxCandidate::Pour(tc) => {
                    handle_pour_tx_candidate(
                        self,
                        tc,
                        &mut ctr_state_update,
                        &mut merkle_update,
                        added_cm_count + ledger_cm_count,
                    )
                    .await?
                }
            };

            added_cm_count += cm_count;
        }

        if let Err(err) = self.sync_pool.remove_tcs(&tcs).await {
            warn!("Error removing txs into the tx pool, err: {}", err);
        }

        let next_merkle_rt = match merkle_update.get("16_0") {
            Some(r) => r,
            None => return Err(format!("next merkle root is missing").into()),
        };

        let (block, txs) = bc.upgrade(
            next_block_height,
            next_tx_height,
            next_merkle_rt.to_owned(),
        );

        if let Some(_b) = self.get_block(block.get_block_hash())? {
            return Err(format!(
                "This block is already persisted: block_hash: {}",
                block.get_block_hash()
            )
            .into());
        };

        let updated_ledger_cm_count = ledger_cm_count + added_cm_count;

        let block_hash = match self
            .ledger_db
            .schema
            .put_block(
                &block,
                &txs,
                &ctr_state_update,
                &merkle_update,
                updated_ledger_cm_count,
            )
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

        debug!(
            "Success writing block, hash: {}, block_height: {}",
            block_hash.green(),
            block.block_height,
        );

        Ok(Some(block_hash))
    }

    pub async fn sync_block(
        &self,
        block: Block,
        txs: Vec<Tx>,
    ) -> Result<Option<String>, LedgerError> {
        let ledger_cm_count = match self.get_ledger_cm_count().await? {
            Some(h) => h,
            None => {
                warn!(
                    "Total cm count does not exist. Possibly the first block"
                );

                0
            }
        };

        let mut ctr_state_update = CtrStateUpdate::new();
        let mut merkle_update = MerkleUpdate::new();

        let mut added_cm_count: u128 = 0;
        for tx in &txs {
            let cm_count = match tx {
                Tx::Mint(tx) => {
                    handle_mint_tx_candidate(
                        self,
                        &tx.tx_candidate,
                        &mut ctr_state_update,
                        &mut merkle_update,
                        ledger_cm_count,
                    )
                    .await?
                }
                Tx::Pour(tx) => {
                    handle_pour_tx_candidate(
                        self,
                        &tx.tx_candidate,
                        &mut ctr_state_update,
                        &mut merkle_update,
                        ledger_cm_count,
                    )
                    .await?
                }
            };

            added_cm_count += cm_count;
        }

        if let Some(_b) = self.get_block(block.get_block_hash())? {
            return Err(format!(
                "This block is already persisted: block_hash: {}",
                block.get_block_hash()
            )
            .into());
        };

        let updated_ledger_cm_count = ledger_cm_count + added_cm_count;

        let block_hash = self
            .ledger_db
            .schema
            .put_block(
                &block,
                &txs,
                &ctr_state_update,
                &merkle_update,
                updated_ledger_cm_count,
            )
            .await?;

        if let Err(err) = self.sync_pool.insert_block(&block).await {
            warn!("Error inserting block into the sync pool, err: {}", err);
        }

        debug!(
            "Successfully sync block, hash: {}, block_height: {}",
            block_hash.green(),
            block.block_height,
        );

        Ok(Some(block_hash))
    }

    pub fn delete_tx(&self, key: &String) -> Result<(), LedgerError> {
        self.ledger_db.schema.delete_tx(key)
    }
}

async fn process_ctr_state_update(
    apis: &DistLedgerApis,
    ctr_addr: &String,
    data: &[u8],
    tx_ctr_op: TxCtrOp,
    ctr_state_update: &mut CtrStateUpdate,
) -> Result<(), LedgerError> {
    let vm = &apis.vm;

    match tx_ctr_op {
        TxCtrOp::ContractDeploy => {
            let receipt = vm.invoke(&data, CtrFn::Init)?;
            let storage = receipt
                .updated_storage
                .ok_or("Contract state needs to be initialized")?;

            ctr_state_update.insert(ctr_addr.clone(), storage);
        }

        TxCtrOp::ContractCall => {
            let req = Request::parse(&data)?;

            match req.ctr_call_type {
                CtrCallType::Query => {
                    warn!(
                        "Tx may contain contract 'execute' request, \
                        but not 'query'"
                    );
                }
                CtrCallType::Execute => {
                    let new_state = match ctr_state_update.get(ctr_addr) {
                        Some(previous_state) => {
                            let ctr_wasm = apis
                                .ledger_db
                                .schema
                                .get_ctr_data_by_ctr_addr(&ctr_addr)
                                .await?
                                .ok_or("ctr data (wasm) should exist")?;

                            let ctr_fn =
                                CtrFn::Execute(req, previous_state.to_vec());

                            let receipt = vm.invoke(ctr_wasm, ctr_fn)?;

                            receipt
                                .updated_storage
                                .ok_or("State needs to be updated")?
                        }
                        None => apis.execute_ctr(&ctr_addr, req).await?,
                    };

                    ctr_state_update
                        .insert(ctr_addr.clone(), new_state.clone());
                }
            };
        }
        TxCtrOp::None => {
            // get `idx` and `height` from tx.`CM`
        }
    };

    Ok(())
}

async fn handle_mint_tx_candidate(
    apis: &DistLedgerApis,
    tc: &MintTxCandidate,
    ctr_state_update: &mut CtrStateUpdate,
    merkle_update: &mut MerkleUpdate,
    ledger_cm_count: u128,
) -> Result<u128, LedgerError> {
    let ctr_addr = &tc.ctr_addr;
    let data = &tc.data;
    let tx_ctr_op = tc.get_ctr_op();

    process_ctr_state_update(apis, ctr_addr, data, tx_ctr_op, ctr_state_update)
        .await?;

    let cm_count = process_merkle_update(
        apis,
        merkle_update,
        vec![&tc.cm],
        ledger_cm_count,
    )
    .await?;

    Ok(cm_count)
}

async fn handle_pour_tx_candidate(
    apis: &DistLedgerApis,
    tc: &PourTxCandidate,
    ctr_state_update: &mut CtrStateUpdate,
    merkle_update: &mut MerkleUpdate,
    ledger_cm_count: u128,
) -> Result<u128, LedgerError> {
    let ctr_addr = &tc.ctr_addr;
    let data = &tc.data;
    let tx_ctr_op = tc.get_ctr_op();

    process_ctr_state_update(apis, ctr_addr, data, tx_ctr_op, ctr_state_update)
        .await?;

    let cm_count = process_merkle_update(
        apis,
        merkle_update,
        vec![&tc.cm_1, &tc.cm_2],
        ledger_cm_count,
    )
    .await?;

    Ok(cm_count)
}

async fn process_merkle_update(
    apis: &DistLedgerApis,
    merkle_update: &mut MerkleUpdate,
    cms: Vec<&[u8; 32]>,
    ledger_cm_count: u128,
) -> Result<u128, LedgerError> {
    let cm_count = cms.len() as u128;

    for (idx, cm) in cms.iter().enumerate() {
        let leaf_idx = ledger_cm_count + idx as u128;
        let auth_path = apis.merkle_tree.generate_auth_paths(leaf_idx);

        let leaf_loc = format!("{}_{}", 0, leaf_idx);

        merkle_update.insert(leaf_loc, **cm);

        for (height, path) in auth_path.iter().enumerate() {
            let curr_idx = path.idx;
            let sibling_idx = match path.direction {
                true => path.idx + 1,
                false => path.idx - 1,
            };

            let sibling_loc = format!("{}_{}", height, sibling_idx);
            let sibling_node = match merkle_update.get(&sibling_loc) {
                Some(n) => *n,
                None => apis.get_merkle_node(&sibling_loc).await?,
            };

            let curr_loc = format!("{}_{}", height, curr_idx);
            let curr_node = match merkle_update.get(&curr_loc) {
                Some(n) => *n,
                None => apis.get_merkle_node(&curr_loc).await?,
            };

            let merkle_node =
                apis.hasher.mimc(&curr_node, &sibling_node)?.to_bytes();

            let parent_idx = sak_proofs::get_parent_idx(curr_idx);
            let update_loc = format!("{}_{}", height + 1, parent_idx);

            merkle_update.insert(update_loc, merkle_node);
        }
    }

    Ok(cm_count)
}
