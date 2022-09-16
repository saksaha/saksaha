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
            disc_port: None,
            secret: String::from(
                "224d0898389759f29ad5c9a6472b26fff86b6293889\
                88eec457a88ce50e907a0",
            ),
            public_key_str: String::from(
                "042c8d005bd935597117181d8ceceaef6d1162de78c32856\
                89d0c36c6170634c124f7b9b911553a1f483ec565c199ea29ff1\
                cd641f10c9a5f8c7c4d4a026db6f7b",
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
            miner: sys_run_args.miner,
            mine_interval: sys_run_args.mine_interval,
            node_task_min_interval: sys_run_args.node_task_min_interval,
            peer_register_interval: sys_run_args.peer_register_interval,
        },
        rpc: RPCConfig {
            rpc_port: Some(34419),
        },
    };
}
