use crate::{CtrStateUpdate, LedgerCols, LedgerError, MerkleUpdate, SakLedger};
use colored::Colorize;
use sak_contract_std::{ContractFn, ERROR_PLACEHOLDER};
use sak_crypto::hasher::MiMC;
use sak_crypto::{Bls12, MerkleTree, Proof, ScalarExt};
use sak_ledger_cfg::CM_TREE_DEPTH;
use sak_ledger_testing::DUMMY_SN;
use sak_logger::{debug, info, warn};
use sak_proof::CoinProof;
use sak_store_interface::LedgerInterface;
use sak_types::{
    Block, BlockCandidate, CmIdx, CtrCallType, CtrRequest, MintTxCandidate, PourTxCandidate, Sn,
    Tx, TxCandidate, TxCtrOp, TxHash,
};

impl SakLedger {
    pub async fn _insert_genesis_block(
        &self,
        genesis_block: BlockCandidate,
    ) -> Result<Option<String>, LedgerError> {
        let persisted_gen_block_hash = if let Some(b) = match self.get_block_by_height(&0).await {
            Ok(b) => b,
            Err(err) => return Err(err),
        } {
            let block_hash = b.get_block_hash().to_string();

            info!("Found genesis block, block_hash: {}", block_hash.green(),);

            Some(block_hash)
        } else {
            info!("Genesis block not found, writing");

            let b = match self.write_block(Some(genesis_block)).await {
                Ok(b) => b,
                Err(err) => {
                    return Err(format!("Genesis block failed to write, err: {}", err).into());
                }
            };

            b
        };

        Ok(persisted_gen_block_hash)
    }

    pub async fn _write_block(
        &self,
        bc: Option<BlockCandidate>,
    ) -> Result<Option<String>, LedgerError> {
        let mut bc = match bc {
            Some(bc) => bc,
            None => match self.make_block_candidate().await? {
                Some(bc) => bc,
                None => {
                    // debug!(
                    //     "No txs to write as a block, aborting write_block()",
                    // );

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

        let next_cm_idx = match self.ledger_db.get_latest_cm_idx()? {
            Some(i) => {
                if i >= 2_u128.pow(CM_TREE_DEPTH) {
                    return Err("CM idx exceeded the tree depth".into());
                }
                i + 1
            }
            None => {
                warn!("Cm idx does not exist. Possibly the first block");
                0
            }
        };

        let tc_len = bc.tx_candidates.len();

        let mut ctr_state_update = CtrStateUpdate::new();
        let mut merkle_update = MerkleUpdate::new();

        debug!(
            "write_block, tc count: {}, next_block_height: {}, \
            next_cm_idx: {}",
            tc_len, next_block_height, next_cm_idx,
        );

        self.filter_tx_candidates(&mut bc)?;
        let tcs = &bc.tx_candidates;

        let mut added_cm_count: u128 = 0;

        for tx_candidate in tcs {
            let cm_count = match tx_candidate {
                TxCandidate::Mint(tc) => {
                    self.handle_mint_tx_candidate(
                        tc,
                        &mut ctr_state_update,
                        &mut merkle_update,
                        next_cm_idx + added_cm_count,
                    )
                    .await?
                }
                TxCandidate::Pour(tc) => {
                    self.handle_pour_tx_candidate(
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

        if let Err(err) = self.sync_pool.remove_tcs(tcs).await {
            warn!("Error removing txs into the tx pool, err: {}", err);
        }

        let next_merkle_rt = match merkle_update.get(format!("{}_0", CM_TREE_DEPTH).as_str()) {
            Some(r) => r,
            None => {
                if tcs.is_empty() {
                    warn!("Block contains no valid txs");
                    return Ok(None);
                } else {
                    return Err("next merkle root is missing".into());
                }
            }
        };

        let (block, txs) = bc.upgrade(next_block_height, next_cm_idx, next_merkle_rt.to_owned());

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

        Ok(Some(block_hash))
    }

    pub async fn _write_blocks(
        &self,
        mut blocks: Vec<(Block, Vec<Tx>)>,
    ) -> Result<Vec<String>, LedgerError> {
        let mut block_hashes = vec![];

        blocks.sort_by(|a, b| a.0.block_height.cmp(&b.0.block_height));

        for (block, txs) in blocks {
            let latest_block_height = self.get_latest_block_height()?.unwrap_or(0);

            if block.block_height != (latest_block_height + 1) {
                warn!(
                    "received not continuous block height, block_height: {}, received : {}",
                    latest_block_height, block.block_height
                );

                continue;
            }

            let tx_candidates = txs.into_iter().map(|tx| tx.downgrade()).collect();

            let bc_candidate = BlockCandidate {
                validator_sig: block.validator_sig,
                tx_candidates,
                witness_sigs: block.witness_sigs,
                created_at: block.created_at,
            };

            let block_hash = self.write_block(Some(bc_candidate)).await?;

            if let Some(h) = block_hash {
                block_hashes.push(h);
            }
        }

        Ok(block_hashes)
    }

    pub(crate) fn _verify_merkle_rt(&self, merkle_rt: &[u8; 32]) -> bool {
        let dummy_merkle_rt = sak_ledger_testing::mock_rt_1().unwrap();

        if merkle_rt == &dummy_merkle_rt {
            true
        } else {
            match self
                .ledger_db
                .get::<Vec<u8>>(LedgerCols::EmptyValue, merkle_rt)
            {
                Ok(Some(_)) => true,
                Ok(None) => false,
                Err(_err) => false,
            }
        }
    }

    pub(crate) fn _verify_sn(&self, sn: &Sn) -> Result<bool, LedgerError> {
        if sn == &DUMMY_SN {
            Ok(true)
        } else {
            match self.ledger_db.get::<TxHash>(LedgerCols::TxHashBySN, sn) {
                Ok(Some(_)) => Err(format!("Serial numbers already exists, sns: {:?}", sn).into()),
                Ok(None) => Ok(true),
                Err(_) => {
                    Err(format!("Tx with serial numbers does not exist, sns: {:?}", sn).into())
                }
            }
        }
    }

    pub(crate) fn _verify_proof(&self, tc: &PourTxCandidate) -> Result<bool, LedgerError> {
        let hasher = MiMC::new();

        let mut public_inputs = vec![];

        for merkle_rt in &tc.merkle_rts {
            public_inputs.push(ScalarExt::parse_arr(merkle_rt)?);
        }

        for sn in &tc.sns {
            public_inputs.push(ScalarExt::parse_arr(sn)?);
        }

        for cm in &tc.cms {
            public_inputs.push(ScalarExt::parse_arr(cm)?);
        }

        let pi_des: Proof<Bls12> = match Proof::read(&*tc.pi) {
            Ok(p) => p,
            Err(err) => {
                return Err(format!("Cannot deserialize the pi, err: {:?}", err).into());
            }
        };

        let verification_result = match &tc.merkle_rts.len() {
            2 => CoinProof::verify_proof_2_to_2(pi_des, &public_inputs, &hasher)?,
            _ => {
                // return Err(format!("Not implement yet").into());
                false
            }
        };

        if !verification_result {
            return Err(format!("Failed to verify proof, tc: {}", tc.get_tx_hash()).into());
        };

        Ok(verification_result)
    }

    pub(crate) fn _filter_tx_candidates(&self, bc: &mut BlockCandidate) -> Result<(), LedgerError> {
        bc.tx_candidates.retain(|tx_candidate| match tx_candidate {
            TxCandidate::Mint(_tc) => {
                return true;
            }
            TxCandidate::Pour(tc) => {
                for sn in &tc.sns {
                    match self.verify_sn(&sn) {
                        Ok(b) => b,
                        Err(err) => {
                            warn!("Tx is filtered, hash: {}, err: {}", tc.get_tx_hash(), err);
                            return false;
                        }
                    };
                }

                for merkle_rt in &tc.merkle_rts {
                    match self.verify_merkle_rt(merkle_rt) {
                        true => {}
                        false => {
                            return false;
                        }
                    };
                }

                match self.verify_proof(tc) {
                    Ok(b) => b,
                    Err(err) => {
                        warn!("Tx is filtered, hash: {}, err: {}", tc.get_tx_hash(), err);
                        return false;
                    }
                };

                true
            }
        });

        Ok(())
    }

    pub(crate) async fn _process_ctr_state_update(
        &self,
        ctr_addr: &String,
        data: &[u8],
        tx_ctr_op: TxCtrOp,
        ctr_state_update: &mut CtrStateUpdate,
    ) -> Result<(), LedgerError> {
        match tx_ctr_op {
            TxCtrOp::ContractDeploy => {
                let receipt = self
                    .contract_processor
                    .as_ref()
                    .ok_or("contract_processor should be present")?
                    .invoke(ctr_addr, data, ContractFn::Init)?;

                let updated_ctr_state = receipt
                    .updated_ctr_state
                    .ok_or("Contract state needs to be initialized")?;

                for entry in updated_ctr_state {
                    let (field, value) = entry.to_owned();

                    let key = format!("{}_{}", ctr_addr, field);

                    ctr_state_update.insert(key.clone(), value.clone());
                }
            }

            TxCtrOp::ContractCall => {
                let req = CtrRequest::parse(ctr_addr, data)?;

                match req.ctr_call_type {
                    CtrCallType::Execute => {
                        warn!(
                            "Tx may contain contract 'update' request, \
                            but not 'execute'"
                        );
                    }
                    CtrCallType::Update => {
                        let new_state = match ctr_state_update.get(ctr_addr) {
                            Some(_) => {
                                let ctr_wasm = self
                                    .ledger_db
                                    .get_ctr_data_by_ctr_addr(ctr_addr)
                                    .await?
                                    .ok_or("ctr data (wasm) should exist")?;

                                let ctr_fn = ContractFn::Update(req);

                                let receipt = self
                                    .contract_processor
                                    .as_ref()
                                    .ok_or("contract_processor should be present")?
                                    .invoke(ctr_addr, &ctr_wasm, ctr_fn)?;

                                receipt.result
                                // receipt
                                //     .updated_ctr_state
                                //     .ok_or("State needs to be updated")?
                                // vec![]
                            }
                            None => self.execute_ctr(req).await?,
                        };

                        let maybe_error_placehorder = match &new_state.get(0..6) {
                            Some(ep) => ep.to_owned(),
                            None => {
                                return Err("new_state should be bigger than 6-byte".into());
                            }
                        };

                        // let key = format!("{}_{}", ctr_addr, new_state);

                        if maybe_error_placehorder != ERROR_PLACEHOLDER {
                            // new_state is the `receipt` from invoked contract
                            ctr_state_update.insert(ctr_addr.clone(), new_state.clone());
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

    pub(crate) async fn _handle_mint_tx_candidate(
        &self,
        tc: &MintTxCandidate,
        ctr_state_update: &mut CtrStateUpdate,
        merkle_update: &mut MerkleUpdate,
        next_cm_idx: CmIdx,
    ) -> Result<u128, LedgerError> {
        let ctr_addr = &tc.ctr_addr;
        let data = &tc.data;
        let tx_ctr_op = tc.get_ctr_op();

        self.process_ctr_state_update(ctr_addr, data, tx_ctr_op, ctr_state_update)
            .await?;

        let cm_count = self
            .process_merkle_update(merkle_update, &tc.cms, next_cm_idx)
            .await?;

        Ok(cm_count)
    }

    pub(crate) async fn _handle_pour_tx_candidate(
        &self,
        tc: &PourTxCandidate,
        ctr_state_update: &mut CtrStateUpdate,
        merkle_update: &mut MerkleUpdate,
        next_cm_idx: CmIdx,
    ) -> Result<u128, LedgerError> {
        let ctr_addr = &tc.ctr_addr;
        let data = &tc.data;
        let tx_ctr_op = tc.get_ctr_op();

        self.process_ctr_state_update(ctr_addr, data, tx_ctr_op, ctr_state_update)
            .await?;

        let cm_count = self
            .process_merkle_update(merkle_update, &tc.cms, next_cm_idx)
            .await?;

        Ok(cm_count)
    }

    pub(crate) async fn _process_merkle_update(
        &self,
        merkle_update: &mut MerkleUpdate,
        cms: &Vec<[u8; 32]>,
        next_cm_idx: CmIdx,
    ) -> Result<u128, LedgerError> {
        let cm_count = cms.len() as u128;

        for (idx, cm) in cms.iter().enumerate() {
            let cm_idx = next_cm_idx + idx as u128;
            let auth_path = self.merkle_tree.generate_auth_paths(cm_idx);

            let leaf_loc = format!("{}_{}", 0, cm_idx);
            merkle_update.insert(leaf_loc, *cm);

            let mut curr_idx = cm_idx;
            for (height, path) in auth_path.iter().enumerate() {
                let sibling_node = match merkle_update.get(&path.node_loc) {
                    Some(n) => *n,
                    None => self.get_merkle_node(&path.node_loc).await?,
                };

                let curr_loc = format!("{}_{}", height, curr_idx);
                let curr_node = match merkle_update.get(&curr_loc) {
                    Some(n) => *n,
                    None => self.get_merkle_node(&curr_loc).await?,
                };

                let lv;
                let rv;
                if path.direction {
                    lv = sibling_node;
                    rv = curr_node;
                } else {
                    lv = curr_node;
                    rv = sibling_node;
                }

                let merkle_node = self.hasher.mimc(&lv, &rv)?.to_bytes();

                let parent_idx = MerkleTree::get_parent_idx(curr_idx);
                let update_loc = format!("{}_{}", height + 1, parent_idx);

                merkle_update.insert(update_loc, merkle_node);

                curr_idx = parent_idx;
            }
        }

        Ok(cm_count)
    }
}
