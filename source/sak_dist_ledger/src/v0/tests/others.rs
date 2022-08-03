use super::utils;
use sak_vm::CtrFn;
use sak_vm::VM;

#[tokio::test(flavor = "multi_thread")]
#[should_panic]
async fn test_insert_invalid_contract_to_tx_pool() {
    sak_test_utils::init_test_log();
    sak_test_utils::init_test_config(&vec![String::from("test")]).unwrap();

    let test_wasm = include_bytes!("./test_invalid_contract.wasm").to_vec();

    let vm = VM::init().expect("VM should be initiated");

    let ctr_fn = CtrFn::Init;

    vm.invoke(test_wasm, ctr_fn)
        .expect("This test should panic");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_deploy_ctr_and_invoke_execute_and_query_when_dist_ledger_writes_new_blocks(
) {
    sak_test_utils::init_test_log();
    sak_test_utils::init_test_config(&vec![String::from("test")]).unwrap();

    let dist_ledger = utils::make_dist_ledger().await;

    dist_ledger.run().await;

    println!("\n[+] Block1: Deploying test validator contract");

    dist_ledger
        .apis
        .write_block(utils::make_dummy_block_candidate_1())
        .await
        .expect("Block_1 must be written");

    println!("\n[+] Block2: Execute::add_validator");

    dist_ledger
        .apis
        .write_block(utils::make_dummy_block_candidate_calling_validator_ctr())
        .await
        .expect("Block_2 must be written");

    println!("\n[+] Block3: Query::get_validator");
}
