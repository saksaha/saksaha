use std::sync::Arc;

// pub struct StoreAccessor {
//     ledger: SakLedger,
//     mrs: SakMRS,
// }

// impl StoreAccessor {
//     pub fn new(mrs: SakMRS, ledger: SakLedger) -> Self {
//         StoreAccessor { mrs, ledger }
//     }

//     pub fn get_mrs_data(&self) -> usize {
//         123123
//     }

//     pub fn put_mrs_data(&self, args: PutMrsDataArgs) {
//         let pks: Vec<usize> = // self.ledger.get(MRS_CTR_ADDR, args.slot_ids);
//         vec![1];

//         self.mrs.put_data(pks, args);
//     }
// }

pub trait StoreAccess {
    fn get_mrs_data(&self) -> usize;
}
