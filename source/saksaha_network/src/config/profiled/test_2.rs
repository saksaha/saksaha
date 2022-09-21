use crate::{
    config::{BlockchainConfig, Config, NodeConfig, P2PConfig, RPCConfig},
    SystemRunArgs,
};
use sak_credential::CredentialProfile;
use sak_p2p_addr::{AddrStatus, UnknownAddr};

pub(crate) fn config(sys_run_args: &SystemRunArgs) -> Config {
    let credential_1 = CredentialProfile::test_1();
    let credential_2 = CredentialProfile::test_2();

    return Config {
        blockchain: BlockchainConfig {
            tx_sync_interval: sys_run_args.tx_sync_interval,
            block_sync_interval: sys_run_args.block_sync_interval,
        },
        p2p: P2PConfig {
            disc_table_capacity: sys_run_args.disc_table_capacity,
            disc_task_interval: sys_run_args.disc_task_interval,
            disc_task_queue_capacity: sys_run_args.disc_task_queue_capacity,
            p2p_task_interval: sys_run_args.p2p_task_interval,
            p2p_task_queue_capacity: sys_run_args.p2p_task_queue_capacity,
            p2p_dial_interval: sys_run_args.p2p_dial_interval,
            p2p_max_conn_count: sys_run_args.p2p_max_conn_count,
            p2p_peer_table_capacity: sys_run_args.p2p_peer_table_capacity,
            p2p_port: Some(35521),
            addr_expire_duration: sys_run_args.addr_expire_duration,
            addr_monitor_interval: sys_run_args.addr_monitor_interval,
            disc_dial_interval: sys_run_args.disc_dial_interval,
            disc_port: Some(35520),
            secret: credential_2.secret,
            public_key_str: credential_2.public_key_str,
            bootstrap_addrs: vec![UnknownAddr {
                ip: String::from("127.0.0.1"),
                disc_port: 35518,
                p2p_port: None,
                sig: None,
                public_key_str: Some(credential_1.public_key_str),
                status: AddrStatus::Initialized,
            }],
        },
        node: NodeConfig {
            miner: sys_run_args.miner.or(Some(true)),
            mine_interval: sys_run_args.mine_interval,
            node_task_min_interval: sys_run_args.node_task_min_interval,
            peer_register_interval: sys_run_args.peer_register_interval,
        },
        rpc: RPCConfig {
            rpc_port: Some(34418),
        },
    };
}
