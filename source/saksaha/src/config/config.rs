use super::profiled::ProfiledConfig;
use crate::{pconfig::PConfig, system::SystemArgs};
use log::{info, warn};
use logger::{tinfo, twarn};
use p2p_addr::UnknownAddr;

#[derive(Debug)]
pub(crate) struct Config {
    pub(crate) app_prefix: String,
    pub(crate) rpc: RPCConfig,
    pub(crate) p2p: P2PConfig,
    pub(crate) db: DBConfig,
}

#[derive(Debug)]
pub(crate) struct P2PConfig {
    pub(crate) disc_port: Option<u16>,
    pub(crate) disc_dial_interval: Option<u16>,
    pub(crate) disc_table_capacity: Option<u16>,
    pub(crate) disc_task_interval: Option<u16>,
    pub(crate) disc_task_queue_capacity: Option<u16>,
    pub(crate) p2p_task_interval: Option<u16>,
    pub(crate) p2p_task_queue_capacity: Option<u16>,
    pub(crate) p2p_dial_interval: Option<u16>,
    pub(crate) p2p_max_conn_count: Option<u16>,
    pub(crate) p2p_peer_table_capacity: Option<u16>,
    pub(crate) p2p_port: Option<u16>,
    pub(crate) addr_expire_duration: Option<u64>,
    pub(crate) addr_monitor_interval: Option<u64>,
    pub(crate) bootstrap_addrs: Vec<UnknownAddr>,
    pub(crate) secret: String,
    pub(crate) public_key_str: String,
}

#[derive(Debug)]
pub(crate) struct RPCConfig {
    pub(crate) rpc_port: Option<u16>,
}

#[derive(Debug)]
pub(crate) struct DBConfig {}

impl Config {
    pub(crate) fn new(
        app_prefix: String,
        sys_args: &SystemArgs,
        pconfig: PConfig,
        profiled_config: Option<ProfiledConfig>,
    ) -> Result<Config, String> {
        let bootstrap_addrs = {
            let mut addrs = match profiled_config {
                Some(c) => c.p2p.bootstrap_addrs,
                None => vec![],
            };

            if let Some(a) = &pconfig.p2p.bootstrap_addrs {
                addrs = a.clone();
            }

            if let Some(a) = &sys_args.bootstrap_urls {
                addrs = vec![];

                for (idx, addr) in a.iter().enumerate() {
                    let addr = match UnknownAddr::new_from_url(addr.clone()) {
                        Ok(ua) => {
                            info!(
                                "-- [{}] Successfully parsed bootstrap url, {}",
                                idx, addr,
                            );

                            ua
                        }
                        Err(err) => {
                            warn!(
                                "Failed to parse \
                                bootstrap url, url: {}, err: {}",
                                addr, err
                            );

                            break;
                        }
                    };

                    addrs.push(addr);
                }
            }

            addrs
        };

        let conf = Config {
            app_prefix: app_prefix.clone(),
            db: DBConfig {},
            rpc: RPCConfig {
                rpc_port: sys_args.rpc_port,
            },
            p2p: P2PConfig {
                disc_port: sys_args.disc_port,
                disc_dial_interval: sys_args.disc_dial_interval,
                disc_table_capacity: sys_args.disc_table_capacity,
                disc_task_interval: sys_args.disc_task_interval,
                disc_task_queue_capacity: sys_args.disc_task_queue_capacity,
                p2p_task_interval: sys_args.p2p_task_interval,
                p2p_task_queue_capacity: sys_args.p2p_task_queue_capacity,
                p2p_peer_table_capacity: sys_args.p2p_peer_table_capacity,
                p2p_dial_interval: sys_args.p2p_dial_interval,
                p2p_port: sys_args.p2p_port,
                p2p_max_conn_count: sys_args.p2p_max_conn_count,
                addr_expire_duration: sys_args.addr_expire_duration,
                addr_monitor_interval: sys_args.addr_monitor_interval,
                secret: pconfig.p2p.secret.clone(),
                public_key_str: pconfig.p2p.public_key_str.clone(),
                bootstrap_addrs,
            },
        };

        Ok(conf)
    }
}
