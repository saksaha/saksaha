use sak_mrs::{PutMrsDataArgs, SakMRS};
use std::sync::Arc;

pub struct StoreAccessor {
    // ledger,
    mrs: SakMRS,
}

impl StoreAccessor {
    pub fn new(mrs: SakMRS) -> Self {
        StoreAccessor { mrs }
    }

    pub fn get_mrs_data(&self) -> usize {
        123123
    }

    pub fn put_mrs_data(&self, args: PutMrsDataArgs) {
        let pks: Vec<usize> = // self.ledger.get(MRS_CTR_ADDR, args.slot_ids);
        vec![1];

        self.mrs.put_data(pks, args);
    }
}
