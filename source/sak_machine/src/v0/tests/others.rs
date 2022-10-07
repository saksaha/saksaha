use super::utils::DistLedgerTestUtils;
use sak_vm::ContractFn;
use sak_vm::SakVM;

#[tokio::test(flavor = "multi_thread")]
#[should_panic]
async fn test_insert_invalid_contract_to_tx_pool() {
    DistLedgerTestUtils::init_saksaha_test();

    let test_wasm = include_bytes!("./assets/test_invalid_contract.wasm").to_vec();

    let vm = SakVM::init().expect("VM should be initiated");

    let ctr_fn = ContractFn::Init;

    vm.invoke(test_wasm, ctr_fn)
        .expect("This test should panic");
}
