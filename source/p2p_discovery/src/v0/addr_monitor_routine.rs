use super::AddrTable;
use crate::DiscAddr;
use logger::tdebug;
use std::sync::Arc;
use std::time::Duration;

pub(crate) struct AddrMonitorRoutine {
    pub(crate) addr_monitor_interval: Duration,
    pub(crate) addr_table: Arc<AddrTable>,
}

impl AddrMonitorRoutine {
    pub async fn run(&self) {
        let rest_after_one_iteration = self.addr_monitor_interval * 5;

        loop {
            let table = self.addr_table.clone();

            let addr_map_lock = table.addr_map.read().await;

            let addrs: Vec<Arc<DiscAddr>> =
                addr_map_lock.values().map(|addr| addr.clone()).collect();

            drop(addr_map_lock);

            for (idx, addr) in addrs.iter().enumerate() {
                tdebug!(
                    "p2p_discovery",
                    "monitor",
                    "addr status [{}] - {}",
                    idx,
                    addr.known_addr,
                );
                tokio::time::sleep(self.addr_monitor_interval).await;
            }

            tokio::time::sleep(rest_after_one_iteration).await;
        }
    }
}
