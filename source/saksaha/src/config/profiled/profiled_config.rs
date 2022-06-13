use super::local_1;
use super::local_2;
use sak_p2p_addr::UnknownAddr;

pub(crate) struct ProfiledConfig {
    pub(crate) app_prefix: String,
    pub(crate) p2p: ProfiledP2PConfig,
}

pub(crate) struct ProfiledP2PConfig {
    pub(crate) bootstrap_addrs: Vec<UnknownAddr>,
}

impl ProfiledConfig {
    pub(crate) fn new(cfg_profile: &String) -> Result<Self, String> {
        match cfg_profile.as_ref() {
            "dev_local_1" => Ok(local_1::get_config()),
            "dev_local_2" => Ok(local_2::get_config()),
            _ => {
                return Err(format!(
                    "DevConfig does not exist with the \
                            specified cfg_profile ({})",
                    cfg_profile,
                ));
            }
        }
    }
}
