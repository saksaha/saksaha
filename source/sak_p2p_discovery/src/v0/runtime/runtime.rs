use crate::AddrTable;
use crate::PublicKey;
use sak_logger::debug;
use sak_p2p_addr::AddrStatus;
use std::sync::Arc;
use std::time::Duration;
use std::time::SystemTime;

pub(crate) struct DiscRuntime {
    pub(crate) addr_monitor_interval: Duration,
    pub(crate) addr_table: Arc<AddrTable>,
}

impl DiscRuntime {
    pub async fn run(&self) {
        let rest_after_one_iteration = self.addr_monitor_interval * 5;

        loop {
            let time_since = SystemTime::now();

            let addr_map_lock = self.addr_table.get_addr_map_read().await;

            let public_keys: Vec<PublicKey> =
                addr_map_lock.keys().map(|k| k.clone()).collect();

            drop(addr_map_lock);

            for public_key in public_keys.iter() {
                let _ =
                    drop_address_if_necessary(&self.addr_table, &public_key)
                        .await;

                sak_utils_time::wait_until_min_interval(
                    time_since,
                    self.addr_monitor_interval,
                )
                .await;
            }

            sak_utils_time::wait_until_min_interval(
                time_since,
                rest_after_one_iteration,
            )
            .await;
        }
    }
}

async fn drop_address_if_necessary(
    addr_table: &Arc<AddrTable>,
    public_key: &String,
) -> Result<(), String> {
    let addr = match addr_table.get_mapped_addr(public_key).await {
        Some(p) => p,
        None => return Ok(()),
    };

    let addr_status_lock = addr.known_addr.status.read().await;

    if let AddrStatus::Disconnected = *addr_status_lock {
        let pkey = addr.get_public_key_short();

        debug!(
            "Removing disconnected address from Addr Table, public_key: {}",
            pkey,
        );

        drop(addr_status_lock);

        addr_table.remove_mapping(public_key).await;
    }

    Ok(())
}
