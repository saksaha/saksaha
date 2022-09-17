// use super::profiled::ProfiledConfig;
use super::{dev_local_1, dev_local_2};
use crate::{system::SystemRunArgs, PConfig, PersistedP2PConfig, SaksahaError};
use log::{info, warn};
use sak_crypto::SakKey;
use sak_p2p_addr::UnknownAddr;

#[derive(Debug)]
pub(crate) struct Config {
    // pub(crate) app_prefix: String,
    pub(crate) blockchain: BlockchainConfig,
    pub(crate) node: NodeConfig,
    pub(crate) rpc: RPCConfig,
    pub(crate) p2p: P2PConfig,
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
pub(crate) struct NodeConfig {
    pub(crate) miner: Option<bool>,
    pub(crate) mine_interval: Option<u64>,
    pub(crate) node_task_min_interval: Option<u64>,
    pub(crate) peer_register_interval: Option<u64>,
}

#[derive(Debug)]
pub(crate) struct BlockchainConfig {
    pub(crate) tx_sync_interval: Option<u64>,
    pub(crate) block_sync_interval: Option<u64>,
}

impl Config {
    pub(crate) fn new(
        sys_run_args: &SystemRunArgs,
        pconfig: PConfig,
    ) -> Result<Config, SaksahaError> {
        let bootstrap_addrs = {
            let mut addrs = vec![];

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

        let cfg = Config {
            blockchain: BlockchainConfig {
                tx_sync_interval: None,
                block_sync_interval: None,
            },
            node: NodeConfig {
                miner: sys_run_args.miner,
                mine_interval: sys_run_args.mine_interval,
                node_task_min_interval: sys_run_args.node_task_min_interval,
                peer_register_interval: sys_run_args.peer_register_interval,
            },
            rpc: RPCConfig {
                rpc_port: sys_run_args.rpc_port,
            },
            p2p: P2PConfig {
                disc_port: sys_run_args.disc_port,
                disc_dial_interval: sys_run_args.disc_dial_interval,
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
                bootstrap_addrs,
                secret: pconfig.p2p.secret,
                public_key_str: pconfig.p2p.public_key,
            },
        };

        Ok(cfg)
    }

    pub fn load_profiled(
        cfg_profile: &String,
        sys_run_args: &SystemRunArgs,
    ) -> Result<Config, SaksahaError> {
        match cfg_profile.as_ref() {
            "dev_local_1" => Ok(dev_local_1::config(sys_run_args)),
            "dev_local_2" => Ok(dev_local_2::config(sys_run_args)),
            _ => {
                return Err(format!(
                    "DevConfig does not exist with the \
                    specified cfg_profile ({})",
                    cfg_profile,
                )
                .into());
            }
        }
    }

    pub fn persist(&self, alias: Option<&String>) -> Result<(), SaksahaError> {
        let acc_addr =
            SakKey::create_acc_addr_from_pk_str(&self.p2p.public_key_str);

        let pconfig = PConfig {
            p2p: PersistedP2PConfig {
                secret: self.p2p.secret.to_string(),
                public_key: self.p2p.public_key_str.to_string(),
                acc_addr,
                bootstrap_addrs: Some(self.p2p.bootstrap_addrs.clone()),
                p2p_port: self.p2p.p2p_port,
                disc_port: self.p2p.disc_port,
            },
        };

        pconfig.persist(alias)?;

        Ok(())
    }
}
