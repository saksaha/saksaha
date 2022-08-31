use crate::{CtrStateUpdate, DistLedgerApis, LedgerError, MerkleUpdate};
use colored::Colorize;
use log::{debug, error, info, warn};
use sak_contract_std::{CtrCallType, CtrRequest, Storage, ERROR_PLACEHOLDER};
use sak_crypto::{Bls12, Hasher, Proof, ScalarExt};
use sak_proofs::CoinProof;
use sak_types::{
    Block, BlockCandidate, CmIdx, MintTxCandidate, PourTxCandidate, Tx,
    TxCandidate, TxCtrOp,
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
                Ok(b) => b,
                Err(err) => {
                    return Err(format!(
                        "Genesis block failed to write, err: {}",
                        err.to_string()
                    )
                    .into());
                }
            };

            b
        };

        Ok(persisted_gen_block_hash.to_string())
    }

    pub async fn write_block(
        &self,
        bc: Option<BlockCandidate>,
    ) -> Result<String, LedgerError> {
        let mut bc = match bc {
            Some(bc) => bc,
            None => match self.make_block_candidate().await? {
                Some(bc) => bc,
                None => {
                    return Err(format!(
                        "No txs to write as a block, aborting"
                    )
                    .into());
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

        let next_cm_idx = match self.ledger_db.get_latest_cm_idx()? {
            Some(i) => i + 1,
            None => {
                warn!("Cm idx does not exist. Possibly the first block");
                0
            }
        };

        let tcs = &bc.tx_candidates.clone();

        let mut ctr_state_update = CtrStateUpdate::new();
        let mut merkle_update = MerkleUpdate::new();

        debug!(
            "write_block, tc count: {}, next_block_height: {}, \
            next_cm_idx: {}",
            tcs.len(),
            next_block_height,
            next_cm_idx,
        );

        self.filter_tx_candidates(&mut bc, tcs)?;
        let tcs = &bc.tx_candidates;

        let mut added_cm_count: u128 = 0;

        for tx_candidate in tcs {
            let cm_count = match tx_candidate {
                TxCandidate::Mint(tc) => {
                    handle_mint_tx_candidate(
                        self,
                        tc,
                        &mut ctr_state_update,
                        &mut merkle_update,
                        next_cm_idx + added_cm_count,
                    )
                    .await?
                }
                TxCandidate::Pour(tc) => {
                    handle_pour_tx_candidate(
                        self,
                        tc,
                        &mut ctr_state_update,
                        &mut merkle_update,
                        next_cm_idx + added_cm_count,
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
            // next_tx_height,
            next_cm_idx,
            next_merkle_rt.to_owned(),
        );

        if let Some(_b) = self.get_block(block.get_block_hash())? {
            return Err(format!(
                "This block is already persisted: block_hash: {}",
                block.get_block_hash()
            )
            .into());
        };

        let block_hash = self
            .ledger_db
            .put_block(&block, &txs, &ctr_state_update, &merkle_update)
            .await?;

        if let Err(err) = self.sync_pool.insert_block(&block).await {
            warn!("Error inserting block into the sync pool, err: {}", err);
        }

        debug!(
            "Success writing block, hash: {}, block_height: {}",
            block_hash.green(),
            block.block_height,
        );

        Ok(block_hash)
    }

    pub async fn write_blocks(
        &self,
        mut blocks: Vec<(Block, Vec<Tx>)>,
        // txs: Vec<Tx>,
    ) -> Result<Vec<String>, LedgerError> {
        // let tx_candidates = txs.into_iter().map(|tx| tx.downgrade()).collect();

        // let bc_candidate = BlockCandidate {
        //     validator_sig: block.validator_sig,
        //     tx_candidates,
        //     witness_sigs: block.witness_sigs,
        //     created_at: block.created_at,
        // };

        // match self.write_block(Some(bc_candidate)).await {
        //     Ok(res) => return Ok(res),
        //     Err(err) => {
        //         return Err(format!("Block sync failed, err: {}", err).into());
        //     }
        // }

        let mut block_hashes = vec![];

        blocks.sort_by(|a, b| a.0.block_height.cmp(&b.0.block_height));

        for (block, txs) in blocks {
            let latest_block_height =
                self.get_latest_block_height()?.unwrap_or(0);

            if block.block_height != (latest_block_height + 1) {
                warn!(
                "received not continuous block height, block_height: {}, received : {}",
                latest_block_height,
                block.block_height
            );

                continue;
            }

            let tx_candidates =
                txs.into_iter().map(|tx| tx.downgrade()).collect();

            let bc_candidate = BlockCandidate {
                validator_sig: block.validator_sig,
                tx_candidates,
                witness_sigs: block.witness_sigs,
                created_at: block.created_at,
            };

            let block_hash = self.write_block(Some(bc_candidate)).await?;

            block_hashes.push(block_hash);
        }

        Ok(block_hashes)
    }

    pub fn delete_tx(&self, key: &String) -> Result<(), LedgerError> {
        self.ledger_db.delete_tx(key)
    }

    pub(crate) fn verify_sn(&self, sn: &[u8; 32]) -> bool {
        match self.ledger_db.get_tx_hash_by_sn(sn) {
            Ok(Some(_)) => return false,
            Ok(None) => return true,
            Err(_) => return false,
        }
    }

    pub(crate) fn verify_proof(
        &self,
        tc: &PourTxCandidate,
    ) -> Result<bool, LedgerError> {
        let hasher = Hasher::new();

        let public_inputs = [
            ScalarExt::parse_arr(&tc.merkle_rt)?,
            ScalarExt::parse_arr(&tc.sn_1)?,
            ScalarExt::parse_arr(&tc.cm_1)?,
            ScalarExt::parse_arr(&tc.cm_2)?,
        ];

        let pi_des: Proof<Bls12> = match Proof::read(&*tc.pi) {
            Ok(p) => p,
            Err(err) => {
                return Err(format!(
                    "Cannot deserialize the pi, err: {:?}",
                    err
                )
                .into());
            }
        };

        let verification_result =
            CoinProof::verify_proof_1_to_2(pi_des, &public_inputs, &hasher)?;

        if !verification_result {
            return Err(format!("Failed to verify proof").into());
        };

        Ok(verification_result)
    }

    pub(crate) fn filter_tx_candidates(
        &self,
        bc: &mut BlockCandidate,
        tx_candidates: &Vec<TxCandidate>,
    ) -> Result<(), LedgerError> {
        let mut valid_tx_candidates: Vec<TxCandidate> = vec![];

        for tx_candidate in tx_candidates {
            match tx_candidate {
                TxCandidate::Mint(_tc) => {
                    valid_tx_candidates.push(tx_candidate.to_owned());
                }
                TxCandidate::Pour(tc) => {
                    let is_valid_sn = self.verify_sn(&tc.sn_1);
                    let is_verified_tx = self.verify_proof(tc)?;

                    if is_valid_sn & is_verified_tx {
                        valid_tx_candidates.push(tx_candidate.to_owned());
                    } else {
                        continue;
                    }
                }
            };
        }

        bc.update_tx_candidates(valid_tx_candidates);
        Ok(())
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
            let req = CtrRequest::parse(&data)?;

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

                    println!(
                        "[+] new_state: {:?}",
                        String::from_utf8(new_state.clone())
                    );

                    let maybe_error_placehorder = match &new_state.get(0..6) {
                        Some(ep) => ep.to_owned(),
                        None => {
                            return Err(format!(
                                "new_state should be bigger than 6-byte"
                            )
                            .into());
                        }
                    };

                    if maybe_error_placehorder != &ERROR_PLACEHOLDER {
                        ctr_state_update
                            .insert(ctr_addr.clone(), new_state.clone());
                    }
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
    next_cm_idx: CmIdx,
    // ledger_cm_count: u128,
) -> Result<u128, LedgerError> {
    let ctr_addr = &tc.ctr_addr;
    let data = &tc.data;
    let tx_ctr_op = tc.get_ctr_op();

    process_ctr_state_update(apis, ctr_addr, data, tx_ctr_op, ctr_state_update)
        .await?;

    let cm_count = process_merkle_update(
        apis,
        merkle_update,
        vec![&tc.cm_1],
        // ledger_cm_count,
        next_cm_idx,
    )
    .await?;

    Ok(cm_count)
}

async fn handle_pour_tx_candidate(
    apis: &DistLedgerApis,
    tc: &PourTxCandidate,
    ctr_state_update: &mut CtrStateUpdate,
    merkle_update: &mut MerkleUpdate,
    next_cm_idx: CmIdx,
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
        next_cm_idx,
        // ledger_cm_count,
    )
    .await?;

    Ok(cm_count)
}

async fn process_merkle_update(
    apis: &DistLedgerApis,
    merkle_update: &mut MerkleUpdate,
    cms: Vec<&[u8; 32]>,
    // ledger_cm_count: u128,
    next_cm_idx: CmIdx,
) -> Result<u128, LedgerError> {
    let cm_count = cms.len() as u128;

    for (idx, cm) in cms.iter().enumerate() {
        // let leaf_idx = ledger_cm_count + idx as u128;
        let cm_idx = next_cm_idx + idx as u128;
        let auth_path = apis.merkle_tree.generate_auth_paths(cm_idx);

        let leaf_loc = format!("{}_{}", 0, cm_idx);

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
