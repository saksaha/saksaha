use crate::{mock_pos, SakMachine, SakMachineArgs};
use sak_credential::{Credential as SakCredential, CredentialProfile};
use sak_types::BlockCandidate;

const APP_NAME: &str = "saksaha";

pub async fn mock_dist_ledger(block: BlockCandidate) -> SakMachine {
    let pos = mock_pos();

    let ledger_path = {
        let config_dir = sak_dir::get_config_dir(APP_NAME).unwrap();
        config_dir.join("ledger")
    };

    let mrs_path = {
        let config_dir = sak_dir::get_config_dir(APP_NAME).unwrap();
        config_dir.join("mrs")
    };

    let dist_ledger_args = SakMachineArgs {
        tx_sync_interval: None,
        genesis_block: Some(block),
        consensus: pos,
        block_sync_interval: None,
        ledger_path,
        mrs_path,
    };

    let dist_ledger = SakMachine::init(dist_ledger_args)
        .await
        .expect("Blockchain should be initialized");

    dist_ledger
}

pub async fn mock_dist_ledger_1() -> SakMachine {
    let pos = mock_pos();
    let credential = CredentialProfile::test_1();

    let test_dir = {
        let tempdir = std::env::temp_dir()
            .join("saksaha_test")
            .join(credential.public_key_str);

        std::fs::create_dir_all(&tempdir).unwrap();
        tempdir
    };

    let ledger_path = { test_dir.join("ledger") };

    let mrs_path = { test_dir.join("mrs") };

    let dist_ledger_args = SakMachineArgs {
        tx_sync_interval: None,
        genesis_block: Some(sak_types::mock_block_1()),
        consensus: pos,
        block_sync_interval: None,
        ledger_path,
        mrs_path,
    };

    let dist_ledger = SakMachine::init(dist_ledger_args)
        .await
        .expect("Blockchain should be initialized");

    dist_ledger
}
