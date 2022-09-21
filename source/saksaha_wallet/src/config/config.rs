use super::profiled;
use crate::WalletError;
use colored::Colorize;
use sak_logger::info;
use sak_types::CoinRecord;

#[derive(Debug)]
pub struct Config {
    pub public_key: Option<String>,
    pub secret: Option<String>,
    pub coin_records: Option<Vec<CoinRecord>>,
    pub rpc_port: Option<u16>,
    pub saksaha_endpoint: Option<String>,
}

impl Config {
    pub fn new(cfg_profile: &Option<String>) -> Result<Config, WalletError> {
        let config = if let Some(c) = cfg_profile {
            info!("Loading config, cfg_profile: {}", c.yellow());

            match c.as_str() {
                "dev_local_1" => profiled::dev_local_1()?,
                "dev_local_2" => profiled::dev_local_2()?,
                _ => {
                    return Err(format!("Corresponding profiled config does not exist").into());
                }
            }
        } else {
            Config::empty()
        };

        Ok(config)
    }

    pub fn empty() -> Config {
        Config {
            public_key: None,
            secret: None,
            coin_records: None,
            rpc_port: None,
            saksaha_endpoint: None,
        }
    }
}
