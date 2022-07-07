use async_trait::async_trait;
use sak_contract_std::{CtrCallType, Request};
use sak_dist_ledger::{Consensus, ConsensusError, DistLedger};
use sak_p2p_id::Identity;
use sak_proofs::MerkleTree;
use sak_types::{BlockCandidate, Tx, TxCandidate};
use std::{collections::HashMap, sync::Arc};

pub struct Pos {
    pub validator_ctr_addr: String,
    pub identity: Arc<Identity>,
}

#[async_trait]
impl Consensus for Pos {
    async fn do_consensus(
        &self,
        dist_ledger: &DistLedger,
        tx_candidates: Vec<TxCandidate>,
    ) -> Result<BlockCandidate, ConsensusError> {
        let request = Request {
            req_type: "get_validator".to_string(),
            arg: HashMap::with_capacity(10),
            ctr_call_type: CtrCallType::Query,
        };

        let validator = dist_ledger
            .query_ctr(&self.validator_ctr_addr, request)
            .await?;

        // let block_height =
        //     next_height(dist_ledger.get_latest_block_height().await?);

        // let latest_tx_height =
        //     next_height(dist_ledger.get_latest_tx_height().await?);

        if self.identity.credential.public_key_str == validator {
            let mut txs = vec![];

            // for (ix, tx_candidate) in tx_candidates.iter().enumerate() {
            //     // let tx_height = latest_tx_height + ix as u128;

            //     let tx = Tx::new(
            //         tx_candidate.get_created_at().to_owned(),
            //         tx_candidate.get_data().to_owned(),
            //         tx_candidate.get_author_sig().to_owned(),
            //         tx_candidate.get_pi().to_owned(),
            //         Some(tx_candidate.get_ctr_addr().to_owned()),
            //         // tx_height,
            //     );
            //     txs.push(tx);
            // }

            let bc = BlockCandidate {
                validator_sig: String::from("1"),
                tx_candidates,
                witness_sigs: vec![],
                created_at: String::from("1"),
                // block_height,
                // merkle_root: String::from("1"),
            };

            return Ok(bc);
        }

        return Err("Not a valid validator".into());
    }
}

fn next_height(maybe_height: Option<u128>) -> u128 {
    match maybe_height {
        Some(h) => h + 1,
        None => 0,
    }
}
