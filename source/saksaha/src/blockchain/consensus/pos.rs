use sak_dist_ledger::{Consensus, DistLedger};
use std::future::Future;
use std::pin::Pin;

pub struct Pos {
    pub validator_ctr_addr: String,
}

impl Consensus<Pos> for Pos {
    fn do_consensus(
        &self,
        dist_ledger: DistLedger<Pos>,
    ) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send + 'static>>
    {
        let validator_ctr_addr = self.validator_ctr_addr.clone();

        Box::pin(async move {
            let a = match dist_ledger.get_ctr_state(&validator_ctr_addr).await {
                Ok(s) => s,
                Err(err) => {
                    return Err(err.to_string());
                }
            };

            println!("power: {:?}", a);

            Ok(())
        })
    }
}
