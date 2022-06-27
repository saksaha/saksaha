use sak_contract_std::Request;
use sak_dist_ledger::{Consensus, ConsensusError, DistLedger};
use sak_types::{BlockCandidate, Tx};
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
            let request = Request {
                req_type: String::from("get_validator"),
            };

            let validator = dist_ledger
                .query_ctr(&_self.validator_ctr_addr, request)
                .await?;

            println!("validator: {:?}", validator);

            // if validator == myself {

            // }

            let bc = BlockCandidate {
                validator_sig: String::from("1"),
                transactions: txs,
                witness_sigs: vec![],
                created_at: String::from("1"),
                height: String::from("1"),
            };

            Ok(bc)
        }

        Box::pin(_do_consensus(self, dist_ledger, txs))
    }
}
