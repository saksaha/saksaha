use super::fs;
use crate::fs as crate_fs;
use crate::SaksahaError;
use colored::Colorize;
use sak_crypto::{SakKey, ToEncodedPoint};
use sak_logger::{info, warn};
use sak_p2p_addr::UnknownAddr;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const INDEX_FILE_ALIAS: &str = "default";

#[derive(Serialize, Deserialize, Debug)]
pub struct PConfig {
    pub p2p: PersistedP2PConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PersistedP2PConfig {
    pub secret: String,
    pub public_key: String,
    pub acc_addr: String,
    pub bootstrap_addrs: Option<Vec<UnknownAddr>>,
    pub p2p_port: Option<u16>,
    pub disc_port: Option<u16>,
}

impl PConfig {
    pub fn init(public_key: &Option<String>) -> Result<PConfig, SaksahaError> {
        println!("\n\n>> Initializing persisted config...");

        if let Some(pk) = public_key {
            let config_file_path = fs::get_config_file_path(pk)?;

            println!(
                "\n\n, 
                Persisted config file: {}",
                config_file_path.to_string_lossy(),
            );

            let pconfig = Self::load_pconfig(&config_file_path)?;

            return Ok(pconfig);
        } else {
            let config_dir = crate_fs::config_dir()?;
            let index_file_name = INDEX_FILE_ALIAS.to_uppercase();
            let index_file_path = config_dir.join(index_file_name);

            println!(
                "\nPublic key is not provided. We will seek the persisted \n\
                config in the default location. We will also look for index \n\
                file (s) if any.",
            );

            println!(
                "{}: {} \n{}: {}",
                "    Config directory".cyan(),
                config_dir.to_string_lossy(),
                "    Index file".cyan(),
                index_file_path.to_string_lossy(),
            );

            if index_file_path.exists() {
                println!("\nFound index file.");

                let pk = std::fs::read_to_string(index_file_path)?;
                let config_file_path = fs::get_config_file_path(&pk)?;
                let pconfig = Self::load_pconfig(&config_file_path)?;

                println!("\nPersisted config successfully loaded.",);

                return Ok(pconfig);
            } else {
                println!("\nIndex file not found. Creating new persisted config.");

                let pconfig = PConfig::create_new_config();

                Ok(pconfig)
            }
        }
    }

    pub fn persist(&self, alias: Option<&String>) -> Result<(), SaksahaError> {
        let acc_dir = crate_fs::acc_dir(&self.p2p.public_key)?;

        if acc_dir.exists() {
            println!(
                "{}\n{}: {:?}",
                "Persisted config exists.",
                "    Config dir: ".cyan(),
                acc_dir,
            );
        } else {
            let config_file_path = fs::get_config_file_path(&self.p2p.public_key)?;
            let data = serde_yaml::to_string(&self)?;
            let _ = std::fs::create_dir_all(acc_dir);

            sak_dir::persist(&data, &config_file_path)?;

            println!(
                "\n{}\n{}: {}",
                "Successfully persisted a pconfig.",
                "    Pconfig path: ".cyan(),
                config_file_path.to_string_lossy(),
            );
        }

        let index_file_alias = INDEX_FILE_ALIAS.to_string();
        let alias = alias.unwrap_or(&index_file_alias);
        Self::persist_index_file(alias, &self.p2p.public_key)?;

        Ok(())
    }

    fn load_pconfig(config_file_path: &PathBuf) -> Result<PConfig, SaksahaError> {
        println!(
            "\n\n>> Loading persisted config...\n{}: {}",
            "    Config file path".cyan(),
            config_file_path.to_string_lossy(),
        );

        if config_file_path.exists() {
            let data = sak_dir::load(config_file_path)?;

            let pconfig = serde_yaml::from_slice::<PConfig>(&data)?;

            return Ok(pconfig);
        } else {
            return Err(
                format!("config path does not exist, path: {:?}", config_file_path,).into(),
            );
        }
    }

    fn persist_index_file(cfg_profile: &str, public_key: &String) -> Result<(), SaksahaError> {
        let config_dir = crate_fs::config_dir()?;

        let index_file_name = cfg_profile.to_uppercase();
        let index_file_path = config_dir.join(index_file_name);

        if index_file_path.exists() {
            println!("Config index file already exists");

            let pk = std::fs::read_to_string(&index_file_path)?;

            if &pk != public_key {
                return Err(format!(
                    "public key in the index file does not match, pk: {}, \
                    public_key (cfg_profile): {}",
                    pk, public_key
                )
                .into());
            }
        } else {
            sak_dir::persist(public_key, &index_file_path)?;

            println!(
                "\nSuccessfully persisted an index file for pconfig\n{}: {}",
                "    Index file path".cyan(),
                index_file_path.to_string_lossy(),
            );
        }

        Ok(())
    }

    fn create_new_config() -> PConfig {
        let (sk, pk) = SakKey::generate();

        let secret_str = sak_crypto::encode_hex(&sk.to_bytes() as &[u8]);
        let public_key_str = sak_crypto::encode_hex(&pk.to_encoded_point(false).to_bytes());

        let acc_addr = SakKey::create_acc_addr(&pk);

        let pconf = PConfig {
            p2p: PersistedP2PConfig {
                secret: secret_str,
                public_key: public_key_str,
                acc_addr,
                bootstrap_addrs: None,
                p2p_port: None,
                disc_port: None,
            },
        };

        pconf
    }
}
