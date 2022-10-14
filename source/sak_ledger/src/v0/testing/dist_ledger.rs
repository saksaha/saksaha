// use crate::v0::tests;
// use crate::{mock_pos, DistLedgerTestUtils, SakLedger, SakLedgerArgs};
// use sak_credential::{Credential as SakCredential, CredentialProfile};
// use sak_types::BlockCandidate;
// use sak_vm::SakVM;
// use sak_vm_interface::ContractProcessor;

// const APP_NAME: &str = "saksaha";

// pub async fn mock_dist_ledger(block: BlockCandidate) -> SakLedger {
//     let pos = mock_pos();

//     let ledger_path = {
//         let config_dir = sak_dir::get_config_dir(APP_NAME).unwrap();
//         config_dir.join("ledger")
//     };

//     let mrs_path = {
//         let config_dir = sak_dir::get_config_dir(APP_NAME).unwrap();
//         config_dir.join("mrs")
//     };

//     let vm: ContractProcessor = {
//         let v = SakVM::init().unwrap();
//         Box::new(v)
//     };

//     let dist_ledger_args = SakLedgerArgs {
//         tx_sync_interval: None,
//         genesis_block: Some(block),
//         consensus: pos,
//         block_sync_interval: None,
//         ledger_path,
//         contract_processor: vm,
//         // mrs_path,
//     };

//     let dist_ledger = SakLedger::init(dist_ledger_args)
//         .await
//         .expect("Blockchain should be initialized");

//     dist_ledger
// }

// pub async fn mock_dist_ledger_1() -> SakLedger {
//     let pos = mock_pos();
//     let credential = CredentialProfile::test_1();

//     DistLedgerTestUtils::init_saksaha_test(credential.public_key_str.clone());

//     let test_dir = {
//         let tempdir = std::env::temp_dir()
//             .join("saksaha_test")
//             .join(credential.public_key_str);

//         std::fs::create_dir_all(&tempdir).unwrap();
//         tempdir
//     };

//     let ledger_path = { test_dir.join("ledger") };

//     let mrs_path = { test_dir.join("mrs") };

//     let vm: ContractProcessor = {
//         let v = SakVM::init().unwrap();
//         Box::new(v)
//     };

//     let dist_ledger_args = SakLedgerArgs {
//         tx_sync_interval: None,
//         genesis_block: Some(sak_types::mock_block_1()),
//         consensus: pos,
//         block_sync_interval: None,
//         ledger_path,
//         contract_processor: vm,
//         // mrs_path,
//     };

//     let dist_ledger = SakLedger::init(dist_ledger_args)
//         .await
//         .expect("Blockchain should be initialized");

//     dist_ledger
// }
