use super::profiled::ProfiledConfig;
use crate::{pconfig::PConfig, system::SystemRunArgs};
use log::{info, warn};
use sak_p2p_addr::UnknownAddr;

#[derive(Debug)]
pub(crate) struct Config {
    pub(crate) app_prefix: String,
    pub(crate) blockchain: BlockchainConfig,
    pub(crate) node: NodeConfig,
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
    pub(crate) p2p_peer_table_capacity: Option<i16>,
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

#[derive(Debug)]
pub(crate) struct NodeConfig {
    pub(crate) miner: bool,
    pub(crate) mine_interval: Option<u64>,
    pub(crate) node_task_min_interval: Option<u64>,
}

#[derive(Debug)]
pub(crate) struct BlockchainConfig {
    pub(crate) tx_sync_interval: Option<u64>,
    pub(crate) block_sync_interval: Option<u64>,
}

impl Config {
    pub(crate) fn new(
        app_prefix: String,
        sys_run_args: &SystemRunArgs,
        pconfig: PConfig,
        profiled_config: ProfiledConfig,
    ) -> Result<Config, String> {
        let bootstrap_addrs = {
            let mut addrs = profiled_config.p2p.bootstrap_addrs.clone();

            if let Some(a) = &pconfig.p2p.bootstrap_addrs {
                addrs = a.clone();
            }

            if let Some(a) = &sys_run_args.bootstrap_urls {
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

        let secret = profiled_config
            .p2p
            .secret
            .unwrap_or(pconfig.p2p.secret.clone());

        let public_key_str = profiled_config
            .p2p
            .public_key_str
            .unwrap_or(pconfig.p2p.public_key.clone());

        let miner = profiled_config.node.miner || sys_run_args.miner;

        let disc_port =
            profiled_config.p2p.disc_port.or(sys_run_args.disc_port);

        let rpc_port = profiled_config.rpc.rpc_port.or(sys_run_args.rpc_port);

        let conf = Config {
            app_prefix: app_prefix.clone(),
            blockchain: BlockchainConfig {
                tx_sync_interval: sys_run_args.tx_sync_interval,
                block_sync_interval: sys_run_args.block_sync_interval,
            },
            node: NodeConfig {
                miner,
                mine_interval: sys_run_args.mine_interval,
                node_task_min_interval: sys_run_args.node_task_min_interval,
            },
            db: DBConfig {},
            rpc: RPCConfig { rpc_port },
            p2p: P2PConfig {
                disc_port,
                disc_dial_interval: sys_run_args.disc_dial_interval,
                disc_table_capacity: sys_run_args.disc_table_capacity,
                disc_task_interval: sys_run_args.disc_task_interval,
                disc_task_queue_capacity: sys_run_args.disc_task_queue_capacity,
                p2p_task_interval: sys_run_args.p2p_task_interval,
                p2p_task_queue_capacity: sys_run_args.p2p_task_queue_capacity,
                p2p_peer_table_capacity: sys_run_args.p2p_peer_table_capacity,
                p2p_dial_interval: sys_run_args.p2p_dial_interval,
                p2p_port: sys_run_args.p2p_port,
                p2p_max_conn_count: sys_run_args.p2p_max_conn_count,
                addr_expire_duration: sys_run_args.addr_expire_duration,
                addr_monitor_interval: sys_run_args.addr_monitor_interval,
                secret,
                public_key_str,
                bootstrap_addrs,
            },
        };

        Ok(conf)
    }
}
