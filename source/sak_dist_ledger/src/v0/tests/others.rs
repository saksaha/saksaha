use super::utils;
use sak_vm::CtrFn;
use sak_vm::VM;

#[tokio::test(flavor = "multi_thread")]
#[should_panic]
async fn test_insert_invalid_contract_to_tx_pool() {
    // sak_test_utils::init_test_log();
    // TestUtil::init_test(vec!["test"]);
    utils::init_test(vec!["test"]);

    let test_wasm = include_bytes!("./assets/test_invalid_contract.wasm").to_vec();

    let vm = VM::init().expect("VM should be initiated");

    let ctr_fn = CtrFn::Init;

    vm.invoke(test_wasm, ctr_fn)
        .expect("This test should panic");
}
