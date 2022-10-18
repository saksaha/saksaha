use sak_ledger::SakLedger;
use sak_mrs::{PutMrsDataArgs, SakMRS};
use std::sync::Arc;

pub struct StoreAccessor {
    ledger: SakLedger,
    mrs: SakMRS,
}

impl StoreAccessor {
    pub fn new(mrs: SakMRS, ledger: SakLedger) -> Self {
        StoreAccessor { mrs, ledger }
    }

    pub fn get_mrs_data(&self) -> usize {
        123123
    }

    pub fn put_mrs_data(&self, args: &[u8]) {
        let args: PutMrsDataArgs = serde_json::from_slice(args).unwrap();
        println!("args: {:?}", args);

        let pks: Vec<usize> = // self.ledger.get(MRS_CTR_ADDR, args.slot_ids);
        vec![1];

        self.mrs.put_data(pks, args);
    }
}
