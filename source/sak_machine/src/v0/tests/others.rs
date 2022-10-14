use super::utils::DistLedgerTestUtils;
use sak_contract_std::ContractFn;
use sak_credential::CredentialProfile;
use sak_mrs::SakMRS;
use sak_store_accessor::StoreAccessor;
use sak_vm::SakVM;
use sak_vm_interface::{ContractProcess, ContractProcessor};
use std::sync::Arc;

#[tokio::test(flavor = "multi_thread")]
#[should_panic]
async fn test_insert_invalid_contract_to_tx_pool() {
    let test_wasm = include_bytes!("./assets/test_invalid_contract.wasm").to_vec();

    let vm: ContractProcessor = {
        let v = SakVM::init().expect("VM should be initiated");
        Box::new(v)
    };

    let credential = CredentialProfile::test_1();

    let test_dir = {
        let tempdir = std::env::temp_dir()
            .join("saksaha_test")
            .join(credential.public_key_str);

        std::fs::create_dir_all(&tempdir).unwrap();
        tempdir
    };

    let mrs_path = { test_dir.join("mrs") };

    let mrs = SakMRS::init(mrs_path).await.unwrap();

    // let store_accessor = {
    //     let a = StoreAccessor::new(mrs);
    //     Arc::new(a)
    // };

    let ctr_fn = ContractFn::Init;

    vm.invoke(&test_wasm, ctr_fn)
        .expect("This test should panic");
}
