use sak_contract_std::{ContractFn, CtrCallType, CtrRequest, Storage};
use sak_credential::CredentialProfile;
use sak_logger::SakLogger;
use sak_mrs::{SakMRS, SakMRSArgs};
use sak_store_interface::MRSAccessor;
use sak_validator_contract::AddValidatorParams;
use sak_vm::SakVM;
use sak_vm_interface::ContractProcess;
use std::{collections::HashMap, sync::Arc};

pub(crate) const VALIDATOR: &[u8] =
    include_bytes!("../../../prebuild/sak_validator_contract.postprocess.wasm");

fn get_dummy_validator_1() -> String {
    String::from(
        "\
            aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
            bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb\
            ccccccccccccccccccccccccccccccccc\
            1111111111111111111111111111111\
            ",
    )
}

fn get_dummy_validator_2() -> String {
    String::from(
        "\
            aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
            bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb\
            ccccccccccccccccccccccccccccccccc\
            2222222222222222222222222222222\
            ",
    )
}

fn get_dummy_validator_3() -> String {
    String::from(
        "\
            aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
            bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb\
            ccccccccccccccccccccccccccccccccc\
            3333333333333333333333333333333\
            ",
    )
}

fn get_dummy_validator_4() -> String {
    String::from(
        "\
            aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
            bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb\
            ccccccccccccccccccccccccccccccccc\
            4444444444444444444444444444444\
            ",
    )
}

fn get_test_validator() -> String {
    String::from(
        "\
            045739d074b8722891c307e8e75c9607e0b55a80778b42ef5f4640d4949dbf399\
            2f6083b729baef9e9545c4e95590616fd382662a09653f2a966ff524989ae8c0f\
            ",
    )
}

// fn get_test_validator_state(validators: Vec<String>) -> Storage {
//     let validator_stage = ValidatorStorage { validators };

//     let mut ret = Storage::with_capacity(10);

//     serde_json::to_vec(&validator_stage).unwrap()
// }

#[tokio::test(flavor = "multi_thread")]
async fn test_call_ctr_validator_fn_init() {
    SakLogger::init_test_console().unwrap();

    let mrs_db_path = {
        let config_dir = sak_dir::get_config_dir("SAKSAHA").unwrap();
        config_dir.join("test").join("mrs")
    };

    let mrs = {
        let mrs_args = SakMRSArgs { mrs_db_path };

        let m = SakMRS::init(mrs_args).await.unwrap();
        let m = Box::new(m) as MRSAccessor;
        Arc::new(m)
    };

    let vm = SakVM::init(mrs.clone()).expect("VM should be initiated");

    let credential = CredentialProfile::test_1();

    let test_dir = {
        let tempdir = std::env::temp_dir()
            .join("saksaha_test")
            .join(credential.public_key_str);

        std::fs::create_dir_all(&tempdir).unwrap();
        tempdir
    };

    let ledger_path = { test_dir.join("ledger") };

    let mrs = {
        let mrs_db_path = { test_dir.join("mrs") };

        let mrs_args = SakMRSArgs { mrs_db_path };

        let m = SakMRS::init(mrs_args).await.unwrap();
    };

    let ctr_wasm = VALIDATOR.to_vec();
    let ctr_fn = ContractFn::Init;
    let ctr_addr = "some_addr".to_string();

    // let receipt = vm
    //     .invoke(&ctr_addr, &ctr_wasm, ctr_fn)
    //     .await
    //     .expect("validator should be obtained");

    // let updated_state = receipt
    //     .updated_storage
    //     .ok_or("Init needs to return state")
    //     .unwrap();

    // let ctr_validator_state: ValidatorStorage = serde_json::from_slice(&updated_state).unwrap();

    // let validator_list_expected = vec![get_test_validator()];

    // println!("validator list expected: {:?}", validator_list_expected);
    // println!(
    //     "validator list acquired: {:?}",
    //     ctr_validator_state.validators
    // );

    // assert_eq!(validator_list_expected, ctr_validator_state.validators);
}

// #[tokio::test(flavor = "multi_thread")]
// async fn test_call_ctr_validator_fn_query() {
//     let vm = SakVM::init().expect("VM should be initiated");

//     let test_validator_vec = vec![
//         get_test_validator(),
//         get_dummy_validator_1(),
//         get_dummy_validator_2(),
//         get_dummy_validator_3(),
//     ];

//     let request = CtrRequest {
//         req_type: "get_validator".to_string(),
//         args: vec![],
//         ctr_call_type: CtrCallType::Query,
//     };

//     let storage = get_test_validator_state(test_validator_vec.clone());

//     let mrs_path = "";

//     let mrs = {
//         let m = SakMRS::init(&mrs_path).await.unwrap();
//         m
//     };

//     // let store_accessor = {
//     //     let a = StoreAccessor::new(mrs);
//     //     Arc::new(a)
//     // };

//     let ctr_wasm = VALIDATOR.to_vec();

//     let ctr_fn = ContractFn::Query(request, storage);

//     let receipt = vm
//         .invoke(&ctr_wasm, ctr_fn)
//         .expect("validator should be obtained");

//     let validators: Vec<String> = serde_json::from_slice(&receipt.result).unwrap();

//     println!("validator expected: {:?}", test_validator_vec[0]);

//     println!("validator acquired: {:?}", validators[0]);

//     assert_eq!(test_validator_vec[0], validators[0]);
// }

// #[tokio::test(flavor = "multi_thread")]
// async fn test_call_ctr_validator_fn_execute_add_validator() {
//     let vm = SakVM::init().expect("VM should be initiated");

//     let test_validator_vec = vec![
//         get_test_validator(),
//         get_dummy_validator_1(),
//         get_dummy_validator_2(),
//         get_dummy_validator_3(),
//     ];

//     let credential = CredentialProfile::test_1();

//     let test_dir = {
//         let tempdir = std::env::temp_dir()
//             .join("saksaha_test")
//             .join(credential.public_key_str);

//         std::fs::create_dir_all(&tempdir).unwrap();
//         tempdir
//     };

//     let ledger_path = { test_dir.join("ledger") };

//     let mrs_path = { test_dir.join("mrs") };

//     let mrs = SakMRS::init(mrs_path).await.unwrap();

//     // let store_accessor = {
//     //     let a = StoreAccessor::new(mrs);
//     //     Arc::new(a)
//     // };

//     let (request, storage) = {
//         let req_type = String::from("add_validator");

//         // let mut args = HashMap::with_capacity(10);
//         // args.insert(String::from("validator"), get_dummy_validator_4());

//         let add_validator_params = AddValidatorParams {
//             validator: get_dummy_validator_4(),
//         };

//         let args = serde_json::to_vec(&add_validator_params).unwrap();

//         let request = CtrRequest {
//             req_type,
//             args,
//             ctr_call_type: CtrCallType::Execute,
//         };

//         let storage = get_test_validator_state(test_validator_vec.clone());

//         (request, storage)
//     };

//     let ctr_wasm = VALIDATOR.to_vec();
//     let ctr_fn = ContractFn::Execute(request, storage);

//     let receipt = vm
//         .invoke(&ctr_wasm, ctr_fn)
//         .expect("validator should be obtained");

//     let updated_storage = receipt.updated_storage.unwrap();

//     let validator_storage: ValidatorStorage = serde_json::from_slice(&updated_storage).unwrap();

//     let validators = validator_storage.validators;

//     println!("original validator list: {:?}", test_validator_vec);

//     println!("updated validator list: {:?}", validators);

//     assert!(validators.contains(&get_dummy_validator_4()));
// }
