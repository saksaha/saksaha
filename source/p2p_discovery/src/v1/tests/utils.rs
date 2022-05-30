use crate::Discovery;
use crate::DiscoveryArgs;
use p2p_addr::AddrStatus;
use p2p_addr::UnknownAddr;
use p2p_identity::Credential;
use std::ops::Index;
use std::slice::SliceIndex;
use std::sync::Arc;

pub(super) fn init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[derive(Debug)]
pub struct CliArgs {
    pub secret_key_str: String,
    pub public_key_str: String,
    pub bootstrap_disc_port: u16,
    pub bootstrap_public_key_str: Option<String>,
    pub disc_args_disc_port: Option<u16>,
}

fn get_bootstrap_addrs(
    disc_port: u16,
    public_key_str: Option<String>,
) -> UnknownAddr {
    UnknownAddr {
        ip: String::from("127.0.0.1"),
        disc_port,
        p2p_port: None,
        sig: None,
        public_key_str,
        status: AddrStatus::Initialized,
    }
}

fn get_discovery_args(
    secret_key_str: String,
    public_key_str: String,
    disc_port: Option<u16>,
    bootstrap_addrs: Vec<UnknownAddr>,
) -> DiscoveryArgs {
    DiscoveryArgs {
        disc_dial_interval: None,
        disc_table_capacity: None,
        disc_task_interval: None,
        disc_task_queue_capacity: None,
        addr_expire_duration: None,
        credential: Arc::new(
            Credential::new(secret_key_str, public_key_str).unwrap(),
        ),
        disc_port,
        p2p_port: 1,
        bootstrap_addrs,
    }
}

pub(super) async fn create_client(cli_index: u16) -> Arc<Discovery> {
    let index: usize = cli_index as usize;

    let test_clients = vec![ //
        CliArgs {
            secret_key_str: String::from("secret_key_str"),
            public_key_str: String::from("public_key_str"),
            bootstrap_disc_port: 65535,
            bootstrap_public_key_str: Some(String::from("bootstrap_public_key_str")),
            disc_args_disc_port: Some(65535),
        },

        // 1
        CliArgs {
        secret_key_str: String::from(
            "aa99cfd91cc6f3b541d28f3e0707f9c7bcf05cf495308294786ca450b501b5f2",
        ),
        public_key_str: String::from(
            "04240874d8c323c22a571f735e835ed2f0619893a3989e557b1c9b4c699ac92b8\
             4d0dc478108629c0353f2876941f90d4b36346bcc19c6b625422adffb53b3a6af",
        ),
        bootstrap_disc_port: 35521,
        bootstrap_public_key_str: Some(String::from(
            "04240874d8c323c22a571f735e835ed2f0619893a3989e557b1c9b4c699ac92b8\
             4d0dc478108629c0353f2876941f90d4b36346bcc19c6b625422adffb53b3a6af",
        )),
        disc_args_disc_port: Some(35521),
    },
        // 2
        CliArgs {
        secret_key_str: String::from(
            "445f36a1f7fafce4408b2277a5d009d1f1ba452d3996bfe7136de4adbfa34d61",
        ),
        public_key_str: String::from(
            "04ce80d8c998044270b26eb7597bd92eb188807ace620644a34bf3be145422e61\
             af51724079002c17758c33b88ade2e789a2153c1fd5b808c1f971127c2592009a",
        ),
        bootstrap_disc_port: 35521,
        bootstrap_public_key_str: Some(String::from(
            "04240874d8c323c22a571f735e835ed2f0619893a3989e557b1c9b4c699ac92b8\
             4d0dc478108629c0353f2876941f90d4b36346bcc19c6b625422adffb53b3a6af",
        )),
        disc_args_disc_port: Some(35522),
    },


    ];

    let secret_key_str = test_clients[index].secret_key_str.clone();
    let public_key_str = test_clients[index].public_key_str.clone();

    let bootstrap_addrs = vec![get_bootstrap_addrs(
        test_clients[index].bootstrap_disc_port.clone(),
        test_clients[index].bootstrap_public_key_str.clone(),
    )];

    let disc_args = get_discovery_args(
        secret_key_str,
        public_key_str,
        test_clients[index].disc_args_disc_port.clone(),
        bootstrap_addrs,
    );

    let (p2p_discovery, disc_port) = {
        let (disc, disc_port) = Discovery::init(disc_args)
            .await
            .expect("Discovery should be initailized");

        (Arc::new(disc), disc_port)
    };

    p2p_discovery
}
