use crate::machine::Machine;
use colored::Colorize;
use sak_logger::{error, info, warn};
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};

const MINE_INTERVAL: u64 = 5000;

pub(super) struct Miner {
    pub(super) machine: Arc<Machine>,
    pub(super) mine_interval: Option<u64>,
}

impl Miner {
    pub fn init(machine: Arc<Machine>, mine_interval: Option<u64>) -> Miner {
        Miner {
            machine,
            mine_interval,
        }
    }

    pub async fn run(&mut self) {
        let mine_interval = match self.mine_interval {
            Some(i) => Duration::from_millis(i),
            None => Duration::from_millis(MINE_INTERVAL),
        };

        info!(
            "Starting {}, mine_interval: {:?}",
            "miner".yellow(),
            mine_interval
        );

        loop {
            let time_since = SystemTime::now();

            match self.machine.ledger.dist_ledger.write_block(None).await {
                Ok(_) => (),
                Err(err) => {
                    error!("write_block failed, err: {}", err.to_string());
                }
            };

            sak_utils_time::wait_until_min_interval(time_since, mine_interval).await;
        }
    }
}
