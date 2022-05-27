use crate::AddrVal;
use crate::Discovery;
use crate::DiscoveryArgs;
use p2p_addr::AddrStatus;
use p2p_addr::UnknownAddr;
use p2p_identity::Credential;
use std::sync::Arc;

pub(super) fn init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

pub(super) async fn create_client_1() -> Arc<Discovery> {
    let secret = String::from(
        "aa99cfd91cc6f3b541d28f3e0707f9c7bcf05cf495308294786ca450b501b5f2",
    );

    let public_key_str = String::from(
        "\
            04240874d8c323c22a571f735e835ed2\
            f0619893a3989e557b1c9b4c699ac92b\
            84d0dc478108629c0353f2876941f90d\
            4b36346bcc19c6b625422adffb53b3a6af\
            ",
    );

    let bootstrap_addrs = vec![UnknownAddr {
        ip: String::from("127.0.0.1"),
        disc_port: 35519,
        p2p_port: None,
        sig: None,
        public_key_str: Some(String::from(
            "\
                04ce80d8c998044270b26eb7597bd92\
                eb188807ace620644a34bf3be145422e\
                61af51724079002c17758c33b88ade2e\
                789a2153c1fd5b808c1f971127c2592009a\
                ",
        )),
        status: AddrStatus::Initialized,
    }];

    let credential = {
        let c = Credential::new(secret, public_key_str).unwrap();

        Arc::new(c)
    };

    let disc_args = DiscoveryArgs {
        disc_dial_interval: None,
        disc_table_capacity: None,
        disc_task_interval: None,
        disc_task_queue_capacity: None,
        credential: credential.clone(),
        // p2p_identity: p2p_identity.clone(),
        disc_port: Some(35520),
        p2p_port: 1,
        bootstrap_addrs,
    };

    let (p2p_discovery, disc_port) = {
        let (disc, disc_port) = Discovery::init(disc_args)
            .await
            .expect("Discovery should be initailized");

        (Arc::new(disc), disc_port)
    };

    p2p_discovery
}

pub(super) async fn create_client_2() -> Arc<Discovery> {
    let secret = String::from(
        "445f36a1f7fafce4408b2277a5d009d1f1ba452d3996bfe7136de4adbfa34d61",
    );

    let public_key_str = String::from(
        "\
            04ce80d8c998044270b26eb7597bd92e\
            b188807ace620644a34bf3be145422e6\
            1af51724079002c17758c33b88ade2e7\
            89a2153c1fd5b808c1f971127c2592009a\
            ",
    );

    let bootstrap_addrs = vec![];

    let credential = {
        let c = Credential::new(secret, public_key_str).unwrap();

        Arc::new(c)
    };

    let disc_args = DiscoveryArgs {
        disc_dial_interval: None,
        disc_table_capacity: None,
        disc_task_interval: None,
        disc_task_queue_capacity: None,
        // p2p_identity: p2p_identity.clone(),
        credential: credential.clone(),
        disc_port: Some(35519),
        p2p_port: 2,
        bootstrap_addrs,
    };

    let (p2p_discovery, disc_port) = {
        let (disc, disc_port) = Discovery::init(disc_args)
            .await
            .expect("Discovery should be initailized");

        (Arc::new(disc), disc_port)
    };

    p2p_discovery
}
