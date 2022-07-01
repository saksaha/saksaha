use crate::machine::Machine;
use log::{error, info, warn};
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};

const MINE_INTERVAL: u64 = 5000;

pub(super) struct Miner {
    pub(super) machine: Arc<Machine>,
    pub(super) mine_interval: Option<u64>,
    error_count: usize,
}

impl Miner {
    pub fn init(machine: Arc<Machine>, mine_interval: Option<u64>) -> Miner {
        Miner {
            machine,
            mine_interval,
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

            sak_utils_time::wait_until_min_interval(time_since, mine_interval)
                .await;
        }
    }
}
