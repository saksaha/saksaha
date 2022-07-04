use super::dev_local_1;
use super::dev_local_2;
use crate::config::NodeConfig;
use crate::config::RPCConfig;
use sak_p2p_addr::UnknownAddr;

pub(crate) struct ProfiledConfig {
    pub(crate) app_prefix: String,
    pub(crate) p2p: ProfiledP2PConfig,
    pub(crate) node: NodeConfig,
    pub(crate) rpc: RPCConfig,
}

pub(crate) struct ProfiledP2PConfig {
    pub(crate) disc_port: Option<u16>,
    pub(crate) secret: Option<String>,
    pub(crate) public_key_str: Option<String>,
    pub(crate) bootstrap_addrs: Vec<UnknownAddr>,
}

impl ProfiledConfig {
    pub(crate) fn new(cfg_profile: &String) -> Result<Self, String> {
        match cfg_profile.as_ref() {
            "dev_local_1" => Ok(dev_local_1::get_config()),
            "dev_local_2" => Ok(dev_local_2::get_config()),
            _ => {
                return Err(format!(
                    "DevConfig does not exist with the \
                            specified cfg_profile ({})",
                    cfg_profile,
                ));
            }
        }
    }

    pub(crate) fn new_empty() -> ProfiledConfig {
        ProfiledConfig {
            app_prefix: String::from("default"),
            p2p: ProfiledP2PConfig {
                disc_port: None,
                secret: None,
                public_key_str: None,
                bootstrap_addrs: vec![],
            },
            node: NodeConfig {
                miner: false,
                mine_interval: None,
            },
            rpc: RPCConfig { rpc_port: None },
        }
    }
}
