use sak_contract_std::{CtrCallType, Request, Storage};
use sak_validator::{AddValidatorParams, ValidatorStorage};
use sak_vm::{CtrFn, VM};
use std::collections::HashMap;

pub(crate) const VALIDATOR: &[u8] =
    include_bytes!("../../../prebuild/sak_validator.postprocess.wasm");

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

fn get_test_validator_state(validators: Vec<String>) -> Storage {
    let validator_stage = ValidatorStorage { validators };

    let mut ret = Storage::with_capacity(10);

    // let key = String::from("validators");
    // let value = serde_json::to_vec(&validators_vec).unwrap();

    // ret.insert(key, value);

    // ret
    serde_json::to_vec(&validator_stage).unwrap()
}

#[tokio::test(flavor = "multi_thread")]
async fn test_call_ctr_validator_fn_init() {
    sak_test_utils::init_test_log();

    // sak_test_utils::init_test_config(&vec![String::from("test")]).unwrap();

    let vm = VM::init().expect("VM should be initiated");

    let ctr_wasm = VALIDATOR.to_vec();
    let ctr_fn = CtrFn::Init;

    let validator_list_from_init = vm
        .invoke(ctr_wasm, ctr_fn)
        .expect("validator should be obtained");

    println!("1, {}", validator_list_from_init);

    // let ctr_validator_state: Storage =
    //     serde_json::from_str(validator_list_from_init.as_str()).unwrap();

    // let validator_list_from_ctr: Vec<String> =
    //     serde_json::from_str(ctr_validator_state.get("validators").unwrap())
    //         .unwrap();

    // let validator_list_expected = vec![get_test_validator()];

    // println!("validator list expected: {:?}", validator_list_expected);
    // println!("validator list acquired: {:?}", validator_list_from_ctr);

    // assert_eq!(
    //     validator_list_expected, //
    //     validator_list_from_ctr
    // );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_call_ctr_validator_fn_query() {
    // init();
    let vm = VM::init().expect("VM should be initiated");

    let test_validator_vec = vec![
        get_test_validator(),
        get_dummy_validator_1(),
        get_dummy_validator_2(),
        get_dummy_validator_3(),
    ];

    // Query(request, storage);
    let request = Request {
        req_type: "get_validator".to_string(),
        args: vec![],
        ctr_call_type: CtrCallType::Query,
    };

    let storage = get_test_validator_state(test_validator_vec.clone());

    // ctr_execute(ctr_wasm, ctr_fn)
    let ctr_wasm = VALIDATOR.to_vec();
    let ctr_fn = CtrFn::Query(request, storage);

    let validator_from_fn_query = vm
        .invoke(ctr_wasm, ctr_fn)
        .expect("validator should be obtained");

    println!("validator expected: {:?}", test_validator_vec[0]);
    println!("validator acquired: {:?}", validator_from_fn_query);

    assert_eq!(
        //
        test_validator_vec[0],
        validator_from_fn_query
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_call_ctr_validator_fn_execute_add_validator() {
    // init();
    let vm = VM::init().expect("VM should be initiated");

    let test_validator_vec = vec![
        get_test_validator(),
        get_dummy_validator_1(),
        get_dummy_validator_2(),
        get_dummy_validator_3(),
    ];

    let (request, storage) = {
        let req_type = String::from("add_validator");

        // let mut args = HashMap::with_capacity(10);
        // args.insert(String::from("validator"), get_dummy_validator_4());

        let add_validator_params = AddValidatorParams {
            validator: get_dummy_validator_4(),
        };

        let args = serde_json::to_vec(&add_validator_params).unwrap();

        let request = Request {
            req_type,
            args,
            ctr_call_type: CtrCallType::Execute,
        };

        let storage = get_test_validator_state(test_validator_vec.clone());

        (request, storage)
    };

    let ctr_wasm = VALIDATOR.to_vec();
    let ctr_fn = CtrFn::Execute(request, storage);

    let validator_state_from_fn_execute = vm
        .invoke(ctr_wasm, ctr_fn)
        .expect("validator should be obtained");

    // let validator_state: Storage =
    //     serde_json::from_str(validator_state_from_fn_execute.as_str()).unwrap();

    let validator_storage: ValidatorStorage =
        serde_json::from_str(validator_state_from_fn_execute.as_str()).unwrap();

    // let validators_string = validator_state.get("validators").unwrap();

    // let validators: Vec<String> =
    //     serde_json::from_str(validators_string.as_str()).unwrap();

    let validators = validator_storage.validators;

    println!("original validator list: {:?}", test_validator_vec);
    println!("updated validator list: {:?}", validators);

    assert!(validators.contains(&get_dummy_validator_4()));
}
