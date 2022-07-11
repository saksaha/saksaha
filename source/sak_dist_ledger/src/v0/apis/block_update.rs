use crate::{DistLedger, LedgerError, RtUpdate, StateUpdate};
use log::warn;
use sak_contract_std::{CtrCallType, Request, Storage};
use sak_proofs::Path;
use sak_types::{Block, BlockCandidate, Tx, TxCandidate, TxCoinOp, TxCtrOp};
use sak_vm::CtrFn;

impl DistLedger {
    pub async fn write_block(
        &self,
        bc: Option<BlockCandidate>,
    ) -> Result<Option<String>, LedgerError> {
        println!("write block()!!");

        // block_update::write_block()
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
        let latest_rt = self.get_latest_rt().await?;

        let latest_tx_height = self.get_latest_tx_height().await?;

        let latest_rt = self.get_latest_rt().await?;
        let tcs = &bc.tx_candidates;

        let mut state_updates = StateUpdate::new();
        let mut rt_updates = RtUpdate::new();

        println!("iterating a total of {} tcs", tcs.len());
        for tc in tcs.iter() {
            println!("\niterating tc, {:?}", tc);

            let ctr_addr = tc.get_ctr_addr();
            let data = tc.get_data();
            let (tx_ctr_op, tx_coin_op) = tc.get_tx_op();

            match tx_ctr_op {
                TxCtrOp::ContractDeploy => {
                    let initial_ctr_state =
                        self.vm.invoke(data, CtrFn::Init)?;

                    state_updates.insert(ctr_addr.clone(), initial_ctr_state);
                }

                TxCtrOp::ContractCall => {
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
                TxCtrOp::None => {
                    // get `idx` and `height` from tx.`CM`
                }
            };

            match tx_coin_op {
                TxCoinOp::Mint => {
                    let next_th = match self.get_latest_tx_height().await? {
                        Some(th) => th + 1,
                        None => 0,
                    };

                    let (auth_path, update_path) =
                        sak_proofs::get_auth_path(next_th as u64);

                    for (height, idx) in auth_path.iter().enumerate() {
                        if auth_path.len() - 1 == height {
                            break;
                        }

                        let sibling_loc = format!("{}_{}", height, idx);

                        println!("sibling loc, {}", sibling_loc);

                        let sibling_node =
                            self.get_merkle_node(&sibling_loc).await?;

                        let update_loc = format!(
                            "{}_{}",
                            height + 1,
                            update_path[*idx as usize + 1]
                        );

                        let cm = tc.get_cm();
                        let sib_cm = sibling_node.unwrap_or(String::from("0"));

                        println!(
                            "update loc, {}, hash of two, {:?} and {:?}",
                            update_loc, cm, sib_cm
                        );

                        // cm
                        // self.hasher.mimc2()

                        // rt_updates.insert(update_loc)
                    }
                }
                TxCoinOp::Pour => {}
            }

            // let rt =
            // rt_updates.insert(rt)
        }

        if let Err(err) = self.sync_pool.remove_tcs(&tcs).await {
            warn!("Error removing txs into the tx pool, err: {}", err);
        }

        // [+] After rt_updates has been updated, `bc` is going to be upgraded
        // [-] which means it can be `extract`ed into `block` and `txs`.
        let (block, txs) =
            bc.upgrade(latest_block_height, latest_tx_height, latest_rt);

        if let Some(_b) = self.get_block(block.get_hash())? {
            return Err(format!(
                "This block is already persisted: block_hash: {}",
                block.get_hash()
            )
            .into());
        };

        let block_hash = match self
            .ledger_db
            .write_block(&block, &txs, &state_updates, &rt_updates)
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
}
