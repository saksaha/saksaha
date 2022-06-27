use sak_contract_std::Request;
use sak_dist_ledger::{Consensus, ConsensusError, DistLedger};
use sak_types::{BlockCandidate, Tx};
use sak_vm::FnType;
use std::future::Future;
use std::pin::Pin;

pub struct Pos {
    pub validator_ctr_addr: String,
}

impl Consensus for Pos {
    fn do_consensus<'a>(
        self: &'a Self,
        dist_ledger: &'a DistLedger,
        txs: Vec<Tx>,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<BlockCandidate, ConsensusError>>
                + Send
                + 'a,
        >,
    >
    where
        Self: Sync + 'a,
    {
        async fn _do_consensus(
            _self: &Pos,
            dist_ledger: &DistLedger,
            txs: Vec<Tx>,
        ) -> Result<BlockCandidate, ConsensusError> {
            // let validator_state = dist_ledger
            //     .get_ctr_state(&_self.validator_ctr_addr)
            //     .await?
            //     .ok_or("validator state should exist")?;

            let request = Request {
                req_type: String::from("get_validator"),
            };

            let validator = dist_ledger.exec_ctr(
                &_self.validator_ctr_addr,
                FnType::Query,
                request,
            );

            // if validator == myself {

            // }

            // let bc = BlockCandidate {
            //     validator_sig: String::from("1"),
            //     transactions: txs,
            //     witness_sigs: vec![],
            //     created_at: String::from("1"),
            //     height: String::from("1"),
            // };

            // Ok(bc)
            Err("".into())
        }

        Box::pin(_do_consensus(self, dist_ledger, txs))

        // Box::pin(async {
        //     let a = dist_ledger.get_ctr_state(&self.validator_ctr_addr).await;

        //     let bc = BlockCandidate {
        //         validator_sig: String::from("1"),
        //         transactions: txs,
        //         witness_sigs: vec![],
        //         created_at: String::from("1"),
        //         height: String::from("1"),
        //     };

        //     Ok(bc)
        // })

        // Box::pin(async {
        //     // a;
        //     // let a = get_ctx(dist_ledger);
        //     let request = Request {
        //         req_type: String::from("get_validator"),
        //     };

        //     // dist_ledger;

        //     // dist_ledger.exec_ctr(&validator_ctr_addr, FnType::Query, request);
        //     // a;
        //     // let a = match dist_ledger.get_ctr_state(&validator_ctr_addr).await {
        //     //     Ok(s) => s,
        //     //     Err(err) => {
        //     //         return Err(err.to_string());
        //     //     }
        //     // };

        //     // println!("power: {:?}", a);

        // let bc = BlockCandidate {
        //     validator_sig: String::from("1"),
        //     transactions: txs,
        //     witness_sigs: vec![],
        //     created_at: String::from("1"),
        //     height: String::from("1"),
        // };

        // Ok(bc)
        // })
    }
}
