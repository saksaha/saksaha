use p2p_identity::{addr::Addr, peer::UnknownPeer};

use crate::{p2p::identity::Identity, system::SystemArgs};

use self::default::DefaultConfig;

pub(crate) mod default;

#[derive(Debug)]
pub(crate) struct Config {
    pub(crate) rpc: RPCConfig,
    pub(crate) p2p: P2PConfig,
}

#[derive(Debug)]
pub(crate) struct P2PConfig {
    pub(crate) disc_dial_interval: Option<u16>,
    pub(crate) disc_table_capacity: Option<u16>,
    pub(crate) p2p_dial_interval: Option<u16>,
    pub(crate) bootstrap_urls: Option<Vec<String>>,
    pub(crate) disc_port: Option<u16>,
    pub(crate) p2p_port: Option<u16>,
    pub(crate) identity: Identity,
    pub(crate) bootstrap_addrs: Vec<Addr>,
}

#[derive(Debug)]
pub(crate) struct RPCConfig {
    pub(crate) rpc_port: Option<u16>,
}

impl Config {
    pub(crate) fn new_from_sys_args(sys_args: &SystemArgs) -> Config {
        let dconfig: DefaultConfig = {
            match sys_args.dev_mode.as_deref() {
                Some("dev_local") => DefaultConfig::new_dev_local(),
                Some(&_) => DefaultConfig::new_empty(),
                None => DefaultConfig::new_empty(),
            }
        };

        let identity = &sys_args.pconfig.p2p.identity;
        let pconfig = &sys_args.pconfig;

        let bootstrap_addrs = {
            let mut addrs = dconfig.p2p.bootstrap_addrs;

            if let Some(up) = &pconfig.p2p.bootstrap_addrs {
                // let up = convert_persisted_unknown_peers_into_unknown_peers(up);
                // p = up;
            }
            addrs
        };

        Config {
            rpc: RPCConfig {
                rpc_port: sys_args.rpc_port,
            },
            p2p: P2PConfig {
                disc_dial_interval: sys_args.disc_dial_interval,
                disc_table_capacity: sys_args.disc_table_capacity,
                p2p_dial_interval: sys_args.p2p_dial_interval,
                bootstrap_urls: sys_args.bootstrap_urls.clone(),
                p2p_port: sys_args.p2p_port,
                disc_port: sys_args.disc_port,
                identity: Identity {
                    secret: identity.secret.clone(),
                    public_key: identity.public_key.clone(),
                },
                bootstrap_addrs,
            },
        }
    }

    // fn convert_persisted_unknown_peers_into_unknown_peers(
    //     persisted_unknown_peers: &Vec<PersistedUnknownPeer>,
    // ) -> Vec<UnknownPeer> {
    //     let mut v = vec![];

    //     for up in persisted_unknown_peers {
    //         v.push(UnknownPeer {
    //             ip: up.ip.to_string(),
    //             disc_port: up.disc_port,
    //             p2p_port: up.p2p_port,
    //             secret: up.secret.clone(),
    //             public_key: up.public_key.clone(),
    //         });
    //     }

    //     v
    // }
}
