use crate::{
    config::{BlockchainConfig, Config, NodeConfig, P2PConfig, RPCConfig},
    SystemRunArgs,
};
use sak_credential::CredentialProfile;
use sak_p2p_addr::{AddrStatus, UnknownAddr};

pub(crate) fn mock_config_4() -> Config {
    let credential_2 = CredentialProfile::test_2();
    let credential_4 = CredentialProfile::test_4();

    return Config {
        blockchain: BlockchainConfig {
            tx_sync_interval: None,
            block_sync_interval: None,
        },
        p2p: P2PConfig {
            disc_table_capacity: None,
            disc_task_interval: None,
            disc_task_queue_capacity: None,
            p2p_task_interval: None,
            p2p_task_queue_capacity: None,
            p2p_dial_interval: None,
            p2p_max_conn_count: None,
            p2p_peer_table_capacity: None,
            p2p_port: Some(35525),
            addr_expire_duration: None,
            addr_monitor_interval: None,
            disc_dial_interval: None,
            disc_port: Some(35524),
            secret: credential_4.secret,
            public_key_str: credential_4.public_key_str,
            bootstrap_addrs: vec![UnknownAddr {
                ip: String::from("127.0.0.1"),
                disc_port: 35520,
                p2p_port: None,
                sig: None,
                public_key_str: Some(credential_2.public_key_str),
                status: AddrStatus::Initialized,
            }],
        },
        node: NodeConfig {
            miner: None,
            mine_interval: None,
            node_task_min_interval: None,
            peer_register_interval: None,
        },
        rpc: RPCConfig {
            rpc_port: Some(34421),
        },
    };
}
