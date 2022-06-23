use crate::{blockchain::SystemContract, machine::Machine};
use log::{error, info};
use sak_p2p_id::{Credential, Identity};
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};

const MINE_INTERVAL: u64 = 5000;

pub(super) struct Miner {
    pub(super) machine: Arc<Machine>,
    pub(super) mine_interval: Option<u64>,
    pub(super) credential: Arc<Credential>,
}

impl Miner {
    pub async fn run(&self) {
        let mine_interval = match self.mine_interval {
            Some(i) => Duration::from_millis(i),
            None => Duration::from_millis(MINE_INTERVAL),
        };

        info!("Starting Miner, mine_interval: {:?}", mine_interval);

        let mut system_contract = match SystemContract::init() {
            Ok(s) => s,
            Err(err) => {
                error!(
                    "Fatal error. Error initializing system contract, err: {}",
                    err,
                );

                return;
            }
        };

        match system_contract.set_validator() {
            Ok(s) => s,
            Err(err) => {
                error!(
                    "Fatal error. Error setting system contract, err: {}",
                    err,
                );

                return;
            }
        };

        loop {
            // self.machine.blockchain.call_contract(contract_addr, fn, fn_args)
            let time_since = SystemTime::now();

            let is_next_validator = match system_contract.get_validator() {
                Ok(b) => {
                    println!(
                        "{},
                        {}",
                        &b, &self.credential.public_key_str
                    );
                    b.eq(&self.credential.public_key_str)
                }
                Err(err) => {
                    error!(
                        "Fatal error. Error getting next validator, err: {}",
                        err,
                    );

                    return;
                }
            };

            // info!("is_next_validator: {}", is_next_validator);

            if is_next_validator {
                // let block = Block {
                // miner_signature:,
                // transactions: self.machine.blockchain.tx_pool.new_tx_hashes,
                // signatures:,
                // created_at:,
                // height:,
                // }

                // if let Err(err) = self.machine.blockchain.write_block().await {
                //     error!("Error writing block, err: {}", err);
                // }
            }

            sak_utils_time::wait_until_min_interval(time_since, mine_interval)
                .await;
        }
    }
}
