use crate::machine::Machine;
use log::{error, info};
use sak_p2p_id::{Credential, Identity};
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};
use tokio::sync::RwLock;

const MINE_INTERVAL: u64 = 5000;

pub(super) struct Miner {
    pub(super) machine: Arc<Machine>,
    pub(super) mine_interval: Option<u64>,
    // pub(super) credential: Arc<Credential>,
    pub(super) identity: Arc<Identity>,
    error_count: usize,
}

impl Miner {
    pub fn init(
        machine: Arc<Machine>,
        mine_interval: Option<u64>,
        identity: Arc<Identity>,
    ) -> Miner {
        Miner {
            machine,
            mine_interval,
            identity,
            error_count: 0,
        }
    }

    pub async fn run(&mut self) {
        let mine_interval = match self.mine_interval {
            Some(i) => Duration::from_millis(i),
            None => Duration::from_millis(MINE_INTERVAL),
        };

        info!("Starting Miner, mine_interval: {:?}", mine_interval);

        //     let mut system_contract = match SystemContract::init() {
        //         Ok(s) => s,
        //         Err(err) => {
        //             error!(
        //                 "Fatal error. Error initializing system contract, err: {}",
        //                 err,
        //             );

        //             return;
        //         }
        //     };

        //     match system_contract.set_validator() {
        //         Ok(s) => s,
        //         Err(err) => {
        //             error!(
        //                 "Fatal error. Error setting system contract, err: {}",
        //                 err,
        //             );

        //             return;
        //         }
        //     };

        loop {
            if self.error_count > 5 {
                error!(
                    "VM will shutdown, error count exceeds the threshold, \
                    count: {}",
                    self.error_count
                );

                return;
            }

            // self.machine.blockchain.call_contract(contract_addr, fn, fn_args)
            let time_since = SystemTime::now();

            let next_validator =
                match self.machine.blockchain.get_next_validator().await {
                    Ok(v) => v,
                    Err(err) => {
                        error!("Could not get next validator, fatal error");
                        self.error_count += 1;

                        continue;
                    }
                };

            let is_next_validator =
                next_validator == self.identity.credential.public_key_str;

            // let is_next_validator = match system_contract.get_validator() {
            //     Ok(b) => {
            //         println!(
            //             "{},
            //             {}",
            //             &b, &self.identity.credential.public_key_str
            //         );
            //         b.eq(&self.identity.credential.public_key_str)
            //     }
            //     Err(err) => {
            //         error!(
            //             "Fatal error. Error getting next validator, err: {}",
            //             err,
            //         );

            //         return;
            //     }
            // };

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
