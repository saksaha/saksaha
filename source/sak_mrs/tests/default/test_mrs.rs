// use sak_contract_std::{CtrCallType, CtrRequest, Storage};
// use sak_mrs::{request_type, MutableRecordStorage, ReserveSlotParams, Slot};
// // use sak_mrs::{AddValidatorParams, ValidatorStorage};
// use sak_vm::{CtrFn, VM};
// use std::collections::HashMap;

// pub(crate) const MRS: &[u8] =
//     include_bytes!("../../../prebuild/sak_mrs.postprocess.wasm");

// fn get_mock_public_key() -> String {
//     String::from(
//         "\
//             045739d074b8722891c307e8e75c9607e0b55a80778b42ef5f4640d4949dbf399\
//             2f6083b729baef9e9545c4e95590616fd382662a09653f2a966ff524989ae8c0f\
//             ",
//     )
// }

// fn get_test_mrs_state(slots: Vec<Slot>) -> Storage {
//     let mrs_stage = MutableRecordStorage { slots };

//     serde_json::to_vec(&mrs_stage).unwrap()
// }

// #[tokio::test(flavor = "multi_thread")]
// async fn test_call_ctr_mrs_fn_execute_reserve_slot() {
//     // init();
//     let vm = VM::init().expect("VM should be initiated");

//     let test_mrs_vec = vec![Slot {
//         pk: get_mock_public_key(),
//         timestamp: String::default(),
//     }];

//     let (request, storage) = {
//         let req_type = String::from(request_type::RESERVE);

//         let reserve_slot_params = ReserveSlotParams {
//             public_key: get_mock_public_key(),
//         };

//         let args = serde_json::to_vec(&reserve_slot_params).unwrap();

//         let request = CtrRequest {
//             req_type,
//             args,
//             ctr_call_type: CtrCallType::Execute,
//         };

//         let storage = get_test_mrs_state(test_mrs_vec);

//         (request, storage)
//     };

//     let ctr_wasm = MRS.to_vec();
//     let ctr_fn = CtrFn::Execute(request, storage);

//     let receipt = vm.invoke(ctr_wasm, ctr_fn).expect("mrs should be obtained");

//     let updated_storage = receipt.updated_storage.unwrap();

//     let mrs_storage: MutableRecordStorage =
//         serde_json::from_slice(&updated_storage).unwrap();

//     // let mrss = mrs_storage.mrss;

//     println!("original mrs list: {:?}", get_mock_public_key());

//     println!("updated mrs list: {:?}", mrs_storage);

//     // assert!(mrss.contains(&get_dummy_mrs_4()));
// }
