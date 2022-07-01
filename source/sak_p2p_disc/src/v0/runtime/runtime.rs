use crate::AddrTable;
use crate::DiscAddr;
use crate::PublicKey;
use log::debug;
use std::sync::Arc;
use std::time::Duration;

pub(crate) struct DiscRuntime {
    pub(crate) addr_monitor_interval: Duration,
    pub(crate) addr_table: Arc<AddrTable>,
}

impl DiscRuntime {
    pub async fn run(&self) {
        let rest_after_one_iteration = self.addr_monitor_interval * 5;

        loop {
            let table = self.addr_table.clone();

            let addr_map_lock = table.addr_map.read().await;

            let public_keys: Vec<PublicKey> =
                addr_map_lock.keys().map(|k| k.clone()).collect();

            drop(addr_map_lock);

            for public_key in public_keys.iter() {
                // debug!("addr status [{}] - {}", idx, addr.known_addr,);

                tokio::time::sleep(self.addr_monitor_interval).await;
            }

            tokio::time::sleep(rest_after_one_iteration).await;
        }
    }
}
