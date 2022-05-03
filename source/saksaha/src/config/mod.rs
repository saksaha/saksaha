pub(crate) mod default;

use self::default::DefaultConfig;
use crate::system::SystemArgs;
use logger::{tinfo, twarn};
use p2p_identity::addr::Addr;

#[derive(Debug)]
pub(crate) struct Config {
    pub(crate) rpc: RPCConfig,
    pub(crate) p2p: P2PConfig,
}

#[derive(Debug)]
pub(crate) struct P2PConfig {
    pub(crate) disc_dial_interval: Option<u16>,
    pub(crate) disc_table_capacity: Option<u16>,
    pub(crate) disc_task_interval: Option<u16>,
    pub(crate) disc_task_queue_capacity: Option<u16>,
    pub(crate) p2p_task_interval: Option<u16>,
    pub(crate) p2p_task_queue_capacity: Option<u16>,
    pub(crate) disc_port: Option<u16>,
    pub(crate) p2p_port: Option<u16>,
    pub(crate) bootstrap_addrs: Vec<Addr>,
    pub(crate) secret: String,
    pub(crate) public_key: String,
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

        // let identity = &sys_args.pconfig.p2p.identity;
        let pconfig = &sys_args.pconfig;

        let bootstrap_addrs = {
            let mut addrs = dconfig.p2p.bootstrap_addrs;

            if let Some(a) = &pconfig.p2p.bootstrap_addrs {
                addrs = a.clone();
            }

            if let Some(a) = &sys_args.bootstrap_urls {
                addrs = vec![];

                for (idx, addr) in a.iter().enumerate() {
                    let addr = match Addr::new_from_url(addr.clone()) {
                        Ok(ua) => {
                            tinfo!(
                                "saksaha",
                                "pconfig",
                                "-- [{}] Successfully parsed bootstrap url, {}",
                                idx,
                                addr,
                            );

                            ua
                        }
                        Err(err) => {
                            twarn!(
                                "saksaha",
                                "config",
                                "Failed to parse \
                                bootstrap url, url: {}, err: {}",
                                addr,
                                err
                            );

                            break;
                        }
                    };

                    addrs.push(addr);
                }
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
                disc_task_interval: sys_args.disc_task_interval,
                disc_task_queue_capacity: sys_args.disc_task_queue_capacity,
                p2p_task_interval: sys_args.p2p_task_interval,
                p2p_task_queue_capacity: sys_args.p2p_task_queue_capacity,
                p2p_port: sys_args.p2p_port,
                disc_port: sys_args.disc_port,
                secret: sys_args.pconfig.p2p.secret.clone(),
                public_key: sys_args.pconfig.p2p.public_key.clone(),
                bootstrap_addrs,
            },
        }
    }
}
