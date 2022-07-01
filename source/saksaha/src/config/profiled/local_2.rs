use super::{ProfiledConfig, ProfiledP2PConfig};
use sak_p2p_addr::{AddrStatus, UnknownAddr};

pub(super) fn get_config() -> ProfiledConfig {
    return ProfiledConfig {
        app_prefix: String::from("dev_local_2"),
        p2p: ProfiledP2PConfig {
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
        },
    };
}
