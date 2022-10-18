use crate::v0::tests;
use crate::{mock_pos, SakMachine, SakMachineArgs, SakMachineTestUtils};
use sak_credential::{Credential as SakCredential, CredentialProfile};
use sak_ledger::{ConsensusResolver, SakLedger, SakLedgerArgs};
use sak_types::BlockCandidate;
use sak_vm::SakVM;
use sak_vm_interface::ContractProcessor;

const APP_NAME: &str = "saksaha";

pub async fn mock_machine(block: BlockCandidate) -> SakMachine {
    let pos: ConsensusResolver = mock_pos();
    let credential = CredentialProfile::test_1();

    let ledger_path = {
        let config_dir = sak_dir::get_config_dir(APP_NAME).unwrap();
        config_dir.join("ledger")
    };

    let mrs_path = {
        let config_dir = sak_dir::get_config_dir(APP_NAME).unwrap();
        config_dir.join("mrs")
    };

    let vm: ContractProcessor = {
        let v = SakVM::init().unwrap();
        Box::new(v)
    };

    let sak_ledger_args = SakLedgerArgs {
        tx_sync_interval: None,
        genesis_block: None,
        block_sync_interval: None,
        consensus: pos,
        ledger_path,
        contract_processor: vm,
    };

    let ledger = {
        let l = SakLedger::init(sak_ledger_args).await.unwrap();

        l
    };

    let dist_ledger_args = SakMachineArgs {
        // tx_sync_interval: None,
        // genesis_block: Some(block),
        // consensus: pos,
        // block_sync_interval: None,
        // ledger_path,
        ledger,
        // mrs_path,
        // vm,
    };

    let dist_ledger = SakMachine::init(dist_ledger_args)
        .await
        .expect("Blockchain should be initialized");

    dist_ledger
}

pub async fn mock_machine_1() -> SakMachine {
    let pos = mock_pos();
    let credential = CredentialProfile::test_1();

    SakMachineTestUtils::init_saksaha_test(credential.public_key_str.clone());

    let test_dir = {
        let tempdir = std::env::temp_dir()
            .join("saksaha_test")
            .join(credential.public_key_str);

        std::fs::create_dir_all(&tempdir).unwrap();
        tempdir
    };

    let ledger_path = { test_dir.join("ledger") };

    let mrs_path = { test_dir.join("mrs") };

    let vm: ContractProcessor = {
        let v = SakVM::init().unwrap();
        Box::new(v)
    };

    let sak_ledger_args = SakLedgerArgs {
        tx_sync_interval: None,
        genesis_block: None,
        block_sync_interval: None,
        consensus: pos,
        ledger_path,
        contract_processor: vm,
    };

    let ledger = {
        let l = SakLedger::init(sak_ledger_args).await.unwrap();

        l
    };

    let dist_ledger_args = SakMachineArgs {
        // mrs_path,
        ledger,
    };

    let machine = SakMachine::init(dist_ledger_args)
        .await
        .expect("Blockchain should be initialized");

    machine
}
