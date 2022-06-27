// use crate::{blockchain::consensus::Pos, system::BoxedError};
// use sak_contract_std::Request;
// use sak_dist_ledger::DistLedger;
// use sak_vm::FnType;

// pub(in crate) struct Validator {
//     ctr_addr: String,
// }

// impl Validator {
//     pub(crate) fn init(ctr_addr: String) -> Validator {
//         let v = Validator { ctr_addr };

//         v
//     }

//     pub(crate) async fn get_next_validator(
//         &self,
//         dist_ledger: &DistLedger,
//     ) -> Result<String, BoxedError> {
//         let request = Request {
//             req_type: String::from("get_validator"),
//         };

//         let next_validator = {
//             let v = dist_ledger
//                 .execute_ctr(&self.ctr_addr, FnType::Query, request)
//                 .await?;

//             String::from_utf8(v.to_vec())?
//         };

//         Ok(next_validator)
//     }

//     // pub fn get_validator(&mut self) -> Result<String, BoxedError> {
//     //     let request = Request {
//     //         req_type: "get_validator",
//     //     };

//     //     let request_serialized =
//     //         serde_json::to_value(request).unwrap().to_string();

//     //     let validator = sak_vm::query(
//     //         self.instance,
//     //         &mut self.store,
//     //         self.memory,
//     //         self.storage_ptr,
//     //         self.storage_len,
//     //         request_serialized,
//     //     )?;

//     //     Ok(validator)
//     // }

//     // pub fn set_validator(&mut self) -> Result<(), BoxedError> {
//     //     let mut storage: HashMap<String, String> =
//     //         HashMap::with_capacity(DEFAULT_VALIDATOR_HASHMAP_CAPACITY);

//     //     storage.insert(
//     //         "validators".to_string(),
//     //         serde_json::to_string(&vec![String::from(
//     //             "04715796a40b0d58fc14a3c4ebee21cb\
//     //             806763066a7f1a17adbc256999764443\
//     //             beb8109cfd000718535c5aa27513a2ed\
//     //             afc6e8bdbe7c27edc2980f9bbc25142fc5\
//     //             ",
//     //         )])
//     //         .unwrap()
//     //         .to_string(),
//     //     );

//     //     let storage_serialized =
//     //         serde_json::to_value(storage).unwrap().to_string();

//     //     let (storage_ptr, storage_len) = sak_vm::execute(
//     //         &self.instance,
//     //         &mut self.store,
//     //         storage_serialized,
//     //     )?;

//     //     self.storage_ptr = storage_ptr;
//     //     self.storage_len = storage_len;

//     //     Ok(())
//     // }
// }
