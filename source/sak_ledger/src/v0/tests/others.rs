use super::utils::DistLedgerTestUtils;
use sak_credential::CredentialProfile;
use std::sync::Arc;

// #[tokio::test(flavor = "multi_thread")]
// #[should_panic]
// async fn test_insert_invalid_contract_to_tx_pool() {
//     let test_wasm = include_bytes!("./assets/test_invalid_contract.wasm").to_vec();

//     let vm = SakVM::init().expect("VM should be initiated");

//     let credential = CredentialProfile::test_1();

//     let test_dir = {
//         let tempdir = std::env::temp_dir()
//             .join("saksaha_test")
//             .join(credential.public_key_str);

//         std::fs::create_dir_all(&tempdir).unwrap();
//         tempdir
//     };

//     let mrs_path = { test_dir.join("mrs") };

//     let mrs = SakMRS::init(mrs_path).await.unwrap();

//     let store_accessor = {
//         let a = StoreAccessor::new(mrs);
//         Arc::new(a)
//     };

//     let ctr_fn = ContractFn::Init(store_accessor);

//     vm.invoke(test_wasm, ctr_fn)
//         .expect("This test should panic");
// }
