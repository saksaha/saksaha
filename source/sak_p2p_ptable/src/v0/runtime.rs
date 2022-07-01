use crate::{PTableError, PeerMap, PeerStatus};
use log::debug;
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};
use tokio::sync::RwLock;

pub(crate) struct Runtime {
    pub(crate) peer_map: Arc<RwLock<PeerMap>>,
}

const PEER_TABLE_RUNTIME_INTERVAL: u64 = 3000;

impl Runtime {
    pub async fn run(&self) {
        let peer_table_runtime_interval =
            Duration::from_millis(PEER_TABLE_RUNTIME_INTERVAL);

        loop {
            let time_since = SystemTime::now();

            let public_keys = get_all_public_keys(&self.peer_map).await;

            for pub_key in public_keys {
                let _ = drop_peer_if_necessary(&self.peer_map, &pub_key).await;

                sak_utils_time::wait_until_min_interval(
                    time_since,
                    peer_table_runtime_interval,
                )
                .await;
            }

            sak_utils_time::wait_until_min_interval(
                time_since,
                peer_table_runtime_interval * 5,
            )
            .await;
        }
    }
}

async fn get_all_public_keys(peer_map: &Arc<RwLock<PeerMap>>) -> Vec<String> {
    let peer_map_lock = peer_map.read().await;
    let public_keys = peer_map_lock
        .keys()
        .map(|k| k.clone())
        .collect::<Vec<String>>();

    public_keys
}

async fn drop_peer_if_necessary(
    peer_map: &Arc<RwLock<PeerMap>>,
    public_key: &String,
) -> Result<(), PTableError> {
    let mut peer_map_lock = peer_map.write().await;

    let peer = match peer_map_lock.get(public_key) {
        Some(p) => p,
        None => return Ok(()),
    };

    let peer_status = peer.status.read().await;

    if let PeerStatus::Disconnected = *peer_status {
        let pkey = peer.get_public_key_short();

        debug!(
            "Removing disconnected peer from peer_map, public_key: {}",
            pkey,
        );

        drop(peer_status);

        peer_map_lock.remove(public_key);
    }

    Ok(())
}
