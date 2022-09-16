use crate::{
    config::{BlockchainConfig, Config, NodeConfig, P2PConfig, RPCConfig},
    SystemRunArgs,
};
use sak_p2p_addr::{AddrStatus, UnknownAddr};

pub(crate) fn config(sys_run_args: &SystemRunArgs) -> Config {
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
            p2p_port: sys_run_args.p2p_port,
            addr_expire_duration: sys_run_args.addr_expire_duration,
            addr_monitor_interval: sys_run_args.addr_monitor_interval,
            disc_dial_interval: sys_run_args.disc_dial_interval,
            disc_port: Some(35518),
            secret: String::from(
                "7297b903877a957748b74068d63d6d5661481975240\
                99fc1df5cd9e8814c66c7",
            ),
            public_key_str: String::from(
                "045739d074b8722891c307e8e75c9607e0b55a80778\
                b42ef5f4640d4949dbf3992f6083b729baef9e9545c4\
                e95590616fd382662a09653f2a966ff524989ae8c0f",
            ),
            bootstrap_addrs: vec![
                UnknownAddr {
                    ip: String::from("127.0.0.1"),
                    disc_port: 35518,
                    p2p_port: None,
                    sig: None,
                    public_key_str: Some(String::from(
                        "\
                            04715796a40b0d58fc14a3c4ebee21cb\
                            806763066a7f1a17adbc256999764443\
                            beb8109cfd000718535c5aa27513a2ed\
                            afc6e8bdbe7c27edc2980f9bbc25142fc5\
                            ",
                    )),
                    status: AddrStatus::Initialized,
                },
                UnknownAddr {
                    ip: String::from("127.0.0.1"),
                    disc_port: 35518,
                    p2p_port: None,
                    sig: None,
                    public_key_str: Some(String::from(
                        "\
                            04715796a40b0d58fc14a3c4ebee21cb\
                            806763066a7f1a17adbc256999764443\
                            beb8109cfd000718535c5aa27513a2ed\
                            afc6e8bdbe7c27edc2980f9bbc25142fc5\
                            ",
                    )),
                    status: AddrStatus::Initialized,
                },
            ],
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
