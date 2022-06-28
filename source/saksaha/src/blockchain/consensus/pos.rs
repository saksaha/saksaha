use async_trait::async_trait;
use sak_contract_std::Request;
use sak_dist_ledger::{Consensus, ConsensusError, DistLedger};
use sak_types::{BlockCandidate, Tx};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

pub struct Pos {
    pub validator_ctr_addr: String,
}

#[async_trait]
impl Consensus for Pos {
    async fn do_consensus(
        &self,
        dist_ledger: &DistLedger,
        txs: Vec<Tx>,
    ) -> Result<BlockCandidate, ConsensusError> {
        let request = Request {
            req_type: String::from("get_validator"),
            arg: HashMap::new(),
        };

        let validator = dist_ledger
            .query_ctr(&self.validator_ctr_addr, request)
            .await?;

        println!("validator: {:?}", validator);

        // // if validator == myself {

        // // }

        // let bc = BlockCandidate {
        //     validator_sig: String::from("1"),
        //     transactions: txs,
        //     witness_sigs: vec![],
        //     created_at: String::from("1"),
        //     height: String::from("1"),
        // };

        // Ok(bc)
        return Err("awel".into());
    }
}
