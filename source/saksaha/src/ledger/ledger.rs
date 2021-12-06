use super::db::DB;
use log::{debug, error, info, warn};
use peer::{PeerStore, PeerValue, RegisteredPeerValue};
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};
use tokio::sync::Mutex;
use tokio::io::{AsyncWriteExt};

pub(crate) struct Ledger {
    db: Arc<DB>,
    peer_store: Arc<PeerStore>,
}

impl Ledger {
    pub fn new(peer_store: Arc<PeerStore>) -> Ledger {
        let db = Arc::new(DB {});

        Ledger { peer_store, db }
    }

    pub async fn start(&self) -> Result<(), String> {
        info!("Ledger is started");

        let min_interval = Duration::from_millis(2000);

        let routine = LedgerRoutine::new(min_interval, self.peer_store.clone());
        routine.run();

        Ok(())
    }
}

struct LedgerRoutine {
    is_running: Arc<Mutex<bool>>,
    min_interval: Duration,
    peer_store: Arc<PeerStore>,
}

impl LedgerRoutine {
    pub fn new(
        min_interval: Duration,
        peer_store: Arc<PeerStore>,
    ) -> LedgerRoutine {
        let is_running = Arc::new(Mutex::new(false));

        LedgerRoutine {
            is_running,
            min_interval,
            peer_store,
        }
    }

    pub fn run(&self) {
        info!("P2P handshake routine starts to run");

        let is_running = self.is_running.clone();
        let min_interval = self.min_interval;
        let peer_store = self.peer_store.clone();

        tokio::spawn(async move {
            let mut is_running_lock = is_running.lock().await;
            *is_running_lock = true;
            std::mem::drop(is_running_lock);

            let peer_store = peer_store.clone();

            loop {
                let start = SystemTime::now();

                let map = peer_store.map.lock().await;
                for peer in map.values() {
                    let mut peer_val = peer.value.lock().await;

                    if let PeerValue::Registered(p) = &mut *peer_val {
                        let peer_id = p.transport.peer_id;
                        println!("sync(): peer_id: {:?}", peer_id);
                    }
                }

                match start.elapsed() {
                    Ok(d) => {
                        if d < min_interval {
                            let diff = min_interval - d;
                            tokio::time::sleep(diff).await;
                        }
                    }
                    Err(err) => {
                        error!(
                            "Calculating the time elapsed fail, err: {}",
                            err
                        );

                        tokio::time::sleep(min_interval).await;
                    }
                }
            }

            let mut is_running_lock = is_running.lock().await;
            *is_running_lock = false;
        });
    }

    pub async fn wakeup(&self) {
        let is_running = self.is_running.lock().await;

        if *is_running == false {
            warn!("P2P dial scheduler routine wakes up");

            self.run();
        }
    }
}
