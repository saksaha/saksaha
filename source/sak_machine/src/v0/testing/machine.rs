use crate::v0::tests;
use crate::{mock_pos, SakMachine, SakMachineArgs, SakMachineTestUtils};
use sak_credential::{Credential as SakCredential, CredentialProfile};
use sak_ledger::{ConsensusResolver, SakLedger, SakLedgerArgs};
use sak_mrs::{SakMRS, SakMRSArgs};
use sak_store_interface::{LedgerAccessor, MRSAccessor};
use sak_types::BlockCandidate;
use sak_vm::SakVM;
use sak_vm_interface::ContractProcessor;
use std::sync::Arc;

const APP_NAME: &str = "saksaha";

pub async fn mock_machine(block: BlockCandidate) -> SakMachine {
    let pos: ConsensusResolver = mock_pos();
    let credential = CredentialProfile::test_1();

    let ledger_path = {
        let config_dir = sak_dir::get_config_dir(APP_NAME).unwrap();
        config_dir.join("ledger")
    };

    let mrs_db_path = {
        let config_dir = sak_dir::get_config_dir(APP_NAME).unwrap();
        config_dir.join("mrs")
    };

    let mrs: Arc<MRSAccessor> = {
        let mrs_args = SakMRSArgs { mrs_db_path };

        let m = SakMRS::init(mrs_args).await.unwrap();
        Arc::new(Box::new(m))
    };

    let sak_ledger_args = SakLedgerArgs {
        tx_sync_interval: None,
        genesis_block: None,
        block_sync_interval: None,
        consensus: pos,
        ledger_path,
        // contract_processor: vm,
    };

    let ledger: Arc<LedgerAccessor> = {
        let l = SakLedger::init(sak_ledger_args).await.unwrap();

        Arc::new(Box::new(l))
    };

    let vm: ContractProcessor = {
        let v = SakVM::init(mrs.clone(), ledger.clone()).unwrap();
        Box::new(v)
    };

    let machine_args = SakMachineArgs { ledger, mrs };

    let machine = SakMachine::init(machine_args)
        .await
        .expect("Blockchain should be initialized");

    machine
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

    let mrs_db_path = { test_dir.join("mrs") };

    let mrs: Arc<MRSAccessor> = {
        let mrs_args = SakMRSArgs { mrs_db_path };

        let m = SakMRS::init(mrs_args).await.unwrap();
        Arc::new(Box::new(m))
    };

    let sak_ledger_args = SakLedgerArgs {
        tx_sync_interval: None,
        genesis_block: None,
        block_sync_interval: None,
        consensus: pos,
        ledger_path,
        // contract_processor: vm
    };

    let ledger: Arc<LedgerAccessor> = {
        let l = SakLedger::init(sak_ledger_args).await.unwrap();

        Arc::new(Box::new(l))
    };

    let vm: ContractProcessor = {
        let v = SakVM::init(mrs.clone(), ledger.clone()).unwrap();
        Box::new(v)
    };

    let dist_ledger_args = SakMachineArgs { ledger, mrs };

    let machine = SakMachine::init(dist_ledger_args)
        .await
        .expect("Blockchain should be initialized");

    machine
}
