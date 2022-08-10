use super::{ProfiledConfig, ProfiledP2PConfig};
use crate::config::{NodeConfig, RPCConfig};
use sak_p2p_addr::{AddrStatus, UnknownAddr};

pub(super) fn get_config() -> ProfiledConfig {
    return ProfiledConfig {
        app_prefix: String::from("dev_local_2"),
        p2p: ProfiledP2PConfig {
            disc_port: None,
            secret: None,
            public_key_str: None,
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
        },
        rpc: RPCConfig { rpc_port: None },
    };
}
