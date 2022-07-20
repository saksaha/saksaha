use crate::{CtrStateUpdate, DistLedger, LedgerError, MerkleUpdate};
use colored::Colorize;
use log::{debug, warn};
use sak_contract_std::{CtrCallType, Request, Storage};
use sak_crypto::ScalarExt;
use sak_types::{
    Block, BlockCandidate, MintTxCandidate, PourTxCandidate, Tx, TxCandidate,
    TxCtrOp, U8Array,
};
use sak_vm::CtrFn;

impl DistLedger {
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

        let next_block_height = match self.get_latest_block_height().await? {
            Some(h) => h,
            None => {
                warn!("Block height does not exist. Possibly the first block");
                0
            }
        };

        let total_cm_count = match self.get_total_cm_count().await? {
            Some(h) => h,
            None => {
                warn!(
                    "Total cm count does not exist. Possibly the first block"
                );

                0
            }
        };

        // let next_tx_height = self.get_latest_tx_height().await?.unwrap_or(0);
        let next_tx_height = match self.get_latest_tx_height().await? {
            Some(th) => th + 1,
            None => 0,
        };

        let tcs = &bc.tx_candidates;
        let mut ctr_state_update = CtrStateUpdate::new();
        let mut merkle_update = MerkleUpdate::new();

        println!(
            "write_block, tc count: {}, next_block_height: {}, \
            next_tx_height: {}",
            tcs.len(),
            next_block_height,
            next_tx_height
        );

        for (idx, tx_candidate) in tcs.iter().enumerate() {
            match tx_candidate {
                TxCandidate::Mint(tc) => {
                    let cm_count = handle_mint_tx_candidate(
                        self,
                        tc,
                        &mut ctr_state_update,
                        &mut merkle_update,
                        next_tx_height + idx as u128,
                    )
                    .await?;
                }
                TxCandidate::Pour(tc) => {
                    let cm_count = handle_pour_tx_candidate(
                        self,
                        tc,
                        &mut ctr_state_update,
                        &mut merkle_update,
                        next_tx_height + idx as u128,
                    )
                    .await?;
                }
            };
        }

        if let Err(err) = self.sync_pool.remove_tcs(&tcs).await {
            warn!("Error removing txs into the tx pool, err: {}", err);
        }

        // let next_merkle_rt = match merkle_update.get("31_0") {
        let next_merkle_rt = match merkle_update.get("3_0") {
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

        let block_hash = match self
            .ledger_db
            .put_block(&block, &txs, &ctr_state_update, &merkle_update)
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
            "Success writing block, hash: {}, height: {}",
            block_hash.green(),
            block.block_height,
        );

        Ok(Some(block_hash))
    }

    pub fn delete_tx(&self, key: &String) -> Result<(), LedgerError> {
        self.ledger_db.delete_tx(key)
    }
}

async fn process_ctr_state_update(
    dist_ledger: &DistLedger,
    ctr_addr: &String,
    data: &[u8],
    tx_ctr_op: TxCtrOp,
    ctr_state_update: &mut CtrStateUpdate,
) -> Result<(), LedgerError> {
    let vm = &dist_ledger.vm;

    match tx_ctr_op {
        TxCtrOp::ContractDeploy => {
            let initial_ctr_state = vm.invoke(&data, CtrFn::Init)?;

            ctr_state_update.insert(ctr_addr.clone(), initial_ctr_state);
        }

        TxCtrOp::ContractCall => {
            let req = Request::parse(&data)?;

            match req.ctr_call_type {
                CtrCallType::Query => {
                    dist_ledger.query_ctr(&ctr_addr, req).await?;
                }
                CtrCallType::Execute => {
                    let new_state = match ctr_state_update.get(ctr_addr) {
                        Some(previous_state) => {
                            let previous_state: Storage =
                                sak_contract_std::parse_storage(
                                    previous_state.as_str(),
                                )?;

                            let ctr_wasm = dist_ledger
                                .ledger_db
                                .get_ctr_data_by_ctr_addr(&ctr_addr)
                                .await?
                                .ok_or("ctr data (wasm) should exist")?;

                            let ctr_fn = CtrFn::Execute(req, previous_state);

                            let ret = vm.invoke(ctr_wasm, ctr_fn)?;

                            ret
                        }
                        None => dist_ledger.execute_ctr(&ctr_addr, req).await?,
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
    dist_ledger: &DistLedger,
    tc: &MintTxCandidate,
    ctr_state_update: &mut CtrStateUpdate,
    merkle_update: &mut MerkleUpdate,
    next_tx_height: u128,
) -> Result<usize, LedgerError> {
    let ctr_addr = &tc.ctr_addr;
    let data = &tc.data;
    let tx_ctr_op = tc.get_ctr_op();

    process_ctr_state_update(
        dist_ledger,
        ctr_addr,
        data,
        tx_ctr_op,
        ctr_state_update,
    )
    .await?;

    let cm_count = process_merkle_update(
        dist_ledger,
        merkle_update,
        vec![&tc.cm],
        next_tx_height,
    )
    .await?;

    Ok(cm_count)
}

async fn handle_pour_tx_candidate(
    dist_ledger: &DistLedger,
    tc: &PourTxCandidate,
    ctr_state_update: &mut CtrStateUpdate,
    merkle_update: &mut MerkleUpdate,
    next_tx_height: u128,
) -> Result<usize, LedgerError> {
    let ctr_addr = &tc.ctr_addr;
    let data = &tc.data;
    let tx_ctr_op = tc.get_ctr_op();

    process_ctr_state_update(
        dist_ledger,
        ctr_addr,
        data,
        tx_ctr_op,
        ctr_state_update,
    )
    .await?;

    let cm_count = process_merkle_update(
        dist_ledger,
        merkle_update,
        vec![&tc.cm_1, &tc.cm_2],
        next_tx_height,
    )
    .await?;

    Ok(cm_count)
}

async fn process_merkle_update(
    dist_ledger: &DistLedger,
    merkle_update: &mut MerkleUpdate,
    cms: Vec<&[u8; 32]>,
    next_tx_height: u128,
) -> Result<usize, LedgerError> {
    let cm_count = cms.len();
    let mut curr_tx_height = next_tx_height;

    for cm in cms {
        let auth_path = dist_ledger
            .merkle_tree
            .generate_auth_paths(curr_tx_height as u32);

        for (height, path) in auth_path.iter().enumerate() {
            let sibling_idx = path.idx;
            let sibling_loc = format!("{}_{}", height, sibling_idx);
            let sibling_node = dist_ledger
                .get_merkle_node(&sibling_loc)
                .await?
                .unwrap_or(U8Array::new_empty_32());

            let curr_cm = cm;
            let sib_cm = &sibling_node;
            let merkle_node =
                dist_ledger.hasher.mimc(curr_cm, sib_cm)?.to_bytes();

            let parent_idx = sak_proofs::get_parent_idx(sibling_idx);
            let update_loc = format!("{}_{}", height + 1, parent_idx);

            merkle_update.insert(update_loc, merkle_node);
            curr_tx_height += 1;
        }
    }

    Ok(cm_count)
}
