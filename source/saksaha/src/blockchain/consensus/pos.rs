use async_trait::async_trait;
use sak_contract_std::{CtrCallType, Request};
use sak_dist_ledger::{Consensus, ConsensusError, DistLedger};
use sak_p2p_id::Identity;
use sak_types::{BlockCandidate, Tx};
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
        txs: Vec<Tx>,
    ) -> Result<BlockCandidate, ConsensusError> {
        let request = Request {
            req_type: "get_validator".to_string(),
            arg: HashMap::with_capacity(10),
            ctr_call_type: CtrCallType::Query,
        };

        let validator = dist_ledger
            .query_ctr(&self.validator_ctr_addr, request)
            .await?;

        let height = next_height(dist_ledger.get_last_block_height().await?)?;

        // TODO use identity
        if !validator.is_empty() {
            let bc = BlockCandidate {
                validator_sig: String::from("1"),
                transactions: txs,
                witness_sigs: vec![],
                created_at: String::from("1"),
                height,
            };

            return Ok(bc);
        }

        return Err("Not a valid validator".into());
    }
}

fn next_height(maybe_height: Option<u128>) -> Result<u128, ConsensusError> {
    match maybe_height {
        Some(h) => Ok(h + 1),
        None => Ok(0),
    }
}
