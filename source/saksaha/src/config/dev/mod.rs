use p2p_addr::UnknownAddr;

pub(crate) mod local_1;

pub(crate) struct DevConfig {
    pub(crate) app_prefix: String,
    pub(crate) p2p: DevP2PConfig,
}

pub(crate) struct DevP2PConfig {
    pub(crate) bootstrap_addrs: Vec<UnknownAddr>,
}

impl DevConfig {
    pub(crate) fn new(cfg_profile: &String) -> Result<DevConfig, String> {
        match cfg_profile.as_ref() {
            "dev_local_1" => Ok(DevConfig::dev_local_1()),
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
