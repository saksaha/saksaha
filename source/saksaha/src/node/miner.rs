use crate::machine::Machine;
use log::{error, info, warn};
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

        loop {
            if self.error_count > 5 {
                error!(
                    "VM will shutdown, error count exceeds the threshold, \
                    count: {}",
                    self.error_count
                );

                return;
            }

            let time_since = SystemTime::now();

            match self.machine.blockchain.dist_ledger.write_block(None).await {
                Ok(_) => (),
                Err(err) => {
                    warn!("write_block failed, err: {}", err.to_string());
                }
            };

            // let next_validator =
            //     match self.machine.blockchain.get_next_validator().await {
            //         Ok(v) => v,
            //         Err(err) => {
            //             error!(
            //                 "Could not get next validator, fatal, err: {}",
            //                 err
            //             );
            //             self.error_count += 1;

            //             continue;
            //         }
            //     };

            // println!("next validator: {}", next_validator);

            // let is_next_validator =
            //     next_validator == self.identity.credential.public_key_str;

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

            sak_utils_time::wait_until_min_interval(time_since, mine_interval)
                .await;
        }
    }
}
