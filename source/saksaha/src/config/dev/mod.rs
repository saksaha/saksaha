use p2p_identity::addr::UnknownAddr;

pub(crate) mod local_1;

pub(crate) struct DevConfig {
    pub(crate) app_prefix: String,
    pub(crate) p2p: DevP2PConfig,
}

pub(crate) struct DevP2PConfig {
    pub(crate) bootstrap_addrs: Vec<UnknownAddr>,
}

impl DevConfig {
    pub(crate) fn new(dev_profile: &String) -> Result<DevConfig, String> {
        match dev_profile.as_ref() {
            "local_1" => Ok(DevConfig::local_1()),
            _ => {
                return Err(format!(
                    "DevConfig does not exist with the \
                            specified dev_profile ({})",
                    dev_profile,
                ));
            }
        }
    }
}
