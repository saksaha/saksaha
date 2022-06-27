use sak_contract_std::Request;
use sak_dist_ledger::{Consensus, DistLedger};
use sak_types::{BlockCandidate, Tx};
use sak_vm::FnType;
use std::future::Future;
use std::pin::Pin;

pub struct Pos {
    pub validator_ctr_addr: String,
}

impl Consensus for Pos {
    fn do_consensus(
        &self,
        dist_ledger: &DistLedger,
        txs: Vec<Tx>,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<BlockCandidate, String>>
                + Send
                + 'static,
        >,
    > {
        let validator_ctr_addr = self.validator_ctr_addr.clone();

        let ctr_state = {
            let a = dist_ledger.get_ctr_state(&validator_ctr_addr);
        };

        // let is_next_validator = {};

        Box::pin(async {
            let request = Request {
                req_type: String::from("get_validator"),
            };

            // dist_ledger;

            // dist_ledger.exec_ctr(&validator_ctr_addr, FnType::Query, request);
            // a;
            // let a = match dist_ledger.get_ctr_state(&validator_ctr_addr).await {
            //     Ok(s) => s,
            //     Err(err) => {
            //         return Err(err.to_string());
            //     }
            // };

            // println!("power: {:?}", a);

            let bc = BlockCandidate {
                validator_sig: String::from("1"),
                transactions: txs,
                witness_sigs: vec![],
                created_at: String::from("1"),
                height: String::from("1"),
            };

            Ok(bc)
        })
    }
}
