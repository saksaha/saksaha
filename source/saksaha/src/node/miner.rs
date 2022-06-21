use crate::{blockchain, machine::Machine};
use log::error;
use sak_p2p_id::Identity;
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};

const MINE_INTERVAL: u64 = 5000;

pub(super) struct Miner {
    pub(super) machine: Arc<Machine>,
    pub(super) mine_interval: Option<u64>,
    pub(super) identity: Arc<Identity>,
}

impl Miner {
    pub async fn run(&self) {
        let mine_interval = match self.mine_interval {
            Some(i) => Duration::from_millis(i),
            None => Duration::from_millis(MINE_INTERVAL),
        };

        loop {
            let time_since = SystemTime::now();
            // if self.machine.blockchain.tx_pool.has_diff() {

            let is_next_validator = match blockchain::get_validator() {
                Ok(b) => {
                    println!("[query] query publickey: {}", &b);
                    println!(
                        "[query]    my publickey: {}",
                        &self.identity.credential.public_key_str
                    );

                    b.eq(&self.identity.credential.public_key_str)
                }
                Err(err) => return,
            };

            println!("{}", is_next_validator);

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
