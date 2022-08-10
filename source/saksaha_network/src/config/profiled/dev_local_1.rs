use super::{ProfiledConfig, ProfiledP2PConfig};
use crate::config::{NodeConfig, RPCConfig};
use sak_p2p_addr::{AddrStatus, UnknownAddr};

pub(super) fn get_config() -> ProfiledConfig {
    return ProfiledConfig {
        app_prefix: String::from("dev_local_1"),
        p2p: ProfiledP2PConfig {
            disc_port: Some(35518),
            secret: Some(String::from(
                "7297b903877a957748b74068d63d6d5661481975240\
                99fc1df5cd9e8814c66c7",
            )),
            public_key_str: Some(String::from(
                "045739d074b8722891c307e8e75c9607e0b55a80778\
                b42ef5f4640d4949dbf3992f6083b729baef9e9545c4\
                e95590616fd382662a09653f2a966ff524989ae8c0f",
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
                // UnknownAddr {
                //     ip: String::from("127.0.0.1"),
                //     disc_port: 35519,
                //     p2p_port: None,
                //     sig: None,
                //     public_key_str: Some(String::from(
                //         "\
                //     048e34a97e173aa7d978f3b3d720487c\
                //     d10c64020e334eae6789cbee6f0856b2\
                //     4953989b5067afff361e3a4534abe506\
                //     91bca238e900a4dff496ef0c84400c26f4\
                //     ",
                //     )),
                //     status: AddrStatus::Initialized,
                // },
                // UnknownAddr {
                //     ip: String::from("127.0.0.1"),
                //     disc_port: 35520,
                //     p2p_port: None,
                //     sig: None,
                //     public_key_str: Some(String::from(
                //         "\
                //     0467e86ec86e8a520503818a4fa09724\
                //     08e56917c7cc76a514e781d8036bd923\
                //     8483a3d8bc5917e2592c4ad0181a1606\
                //     58a536841411f7567f6a673f16ef71191f\
                //     ",
                //     )),
                //     status: AddrStatus::Initialized,
                // },
                // UnknownAddr {
                //     ip: String::from("127.0.0.1"),
                //     disc_port: 35521,
                //     p2p_port: None,
                //     sig: None,
                //     public_key_str: Some(String::from(
                //         "\
                //     0431f595b429a2a6d998030f9a7d71df\
                //     7d617109bf70b91a144ed6ef16dcef2d\
                //     0bf285a541f50bd49b3e4ffc534881b1\
                //     ddeb915b8aa13f6f318b191e02d8fad885\
                //     ",
                //     )),
                //     status: AddrStatus::Initialized,
                // },
            ],
            // sec,
        },
        node: NodeConfig {
            miner: true,
            mine_interval: None,
            node_task_min_interval: None,
        },
        rpc: RPCConfig {
            rpc_port: Some(34418),
        },
    };
}
