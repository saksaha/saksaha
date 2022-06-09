use crate::machine::Machine;
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};
use log::error;

const MINE_INTERVAL: u64 = 5000;

pub(super) struct Miner {
    pub(super) machine: Arc<Machine>,
    pub(super) mine_interval: Option<u64>,
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
            let is_next_validator = {
                let _ret = self.machine.blockchain.query_contract();

                true
            };

            if is_next_validator {
                if let Err(err) = self.machine.blockchain.write_block().await {
                    error!("Error writing block, err: {}", err);
                }
            }

            sak_utils_time::wait_until_min_interval(time_since, mine_interval)
                .await;
        }
    }
}
