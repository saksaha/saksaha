use super::{ProfiledConfig, ProfiledP2PConfig};
use crate::config::{NodeConfig, RPCConfig};
use sak_p2p_addr::{AddrStatus, UnknownAddr};

pub(super) fn get_config() -> ProfiledConfig {
    return ProfiledConfig {
        app_prefix: String::from("dev_local_2"),
        p2p: ProfiledP2PConfig {
            disc_port: None,
            secret: Some(String::from(
                "224d0898389759f29ad5c9a6472b26fff86b6293889\
                88eec457a88ce50e907a0",
            )),
            public_key_str: Some(String::from(
                "042c8d005bd935597117181d8ceceaef6d1162de78c32856\
                89d0c36c6170634c124f7b9b911553a1f483ec565c199ea29ff1\
                cd641f10c9a5f8c7c4d4a026db6f7b",
            )),
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
            miner: false,
            mine_interval: None,
            node_task_min_interval: None,
            peer_register_interval: None,
        },
        rpc: RPCConfig { rpc_port: None },
    };
}
