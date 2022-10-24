use async_trait::async_trait;
use sak_contract_std::{CtrCallType, CtrRequest, CtrRequestData};
use sak_ledger::SakLedger;
use sak_ledger::{Consensus, ConsensusError};
use sak_machine::SakMachine;
use sak_p2p_id::Identity;
use sak_types::{BlockCandidate, TxCandidate};
use std::{collections::HashMap, sync::Arc};

pub struct Pos {
    pub validator_ctr_addr: String,
    pub identity: Arc<Identity>,
}

#[async_trait]
impl Consensus for Pos {
    async fn do_consensus(
        &self,
        dist_ledger: &SakLedger,
        tx_candidates: Vec<TxCandidate>,
    ) -> Result<BlockCandidate, ConsensusError> {
        let request = CtrRequest {
            ctr_addr: self.validator_ctr_addr.to_string(),
            req_type: "get_validator".to_string(),
            args: vec![],
            ctr_call_type: CtrCallType::Query,
        };

        let validator = match dist_ledger
            .execute_ctr(
                // &self.validator_ctr_addr,
                request,
            )
            .await
        {
            Ok(v) => v,
            Err(err) => {
                return Err(format!("Error retrieving a validator, err: {}", err).into());
            }
        };

        let validator_str: String = String::from_utf8(validator)?;

        if self.identity.credential.public_key_str == validator_str {
            let bc = BlockCandidate {
                validator_sig: String::from("1"),
                tx_candidates,
                witness_sigs: vec![],
                created_at: String::from("1"),
            };

            return Ok(bc);
        }

        return Err("Not a valid validator".into());
    }
}
