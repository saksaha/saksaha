use crate::pconfig::PConfig;

pub struct SystemArgs {
    pub rpc_port: Option<u16>,
    pub disc_port: Option<u16>,
    pub p2p_port: Option<u16>,
    pub bootstrap_urls: Option<Vec<String>>,
    pub pconfig: PConfig,
}
