mod call;

pub use call::{Call, CallGuard};
use logger::{terr, twarn};
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::sync::{
    mpsc::{self, UnboundedReceiver, UnboundedSender},
    Mutex,
};

pub struct ActiveCalls {
    map: Arc<Mutex<HashMap<String, Arc<Call>>>>,
    call_removal_tx: Arc<UnboundedSender<String>>,
    removal_routine: Arc<RemovalRoutine>,
}

struct RemovalRoutine {
    map: Arc<Mutex<HashMap<String, Arc<Call>>>>,
    call_removal_rx: Mutex<UnboundedReceiver<String>>,
}

unsafe impl Send for RemovalRoutine {}
unsafe impl Sync for RemovalRoutine {}

impl ActiveCalls {
    pub async fn init() -> ActiveCalls {
        let map = {
            let m = HashMap::new();

            Arc::new(Mutex::new(m))
        };

        let (call_removal_tx, removal_routine) = {
            let (call_removal_tx, call_removal_rx) = {
                let (tx, rx) = mpsc::unbounded_channel();
                (Arc::new(tx), Mutex::new(rx))
            };

            let removal_routine = {
                let r = RemovalRoutine {
                    map: map.clone(),
                    call_removal_rx,
                };

                Arc::new(r)
            };

            let removal_routine_clone = removal_routine.clone();

            tokio::spawn(async move {
                removal_routine_clone.run().await;
            });

            // Wait for a moment to ensure the removal routine be running
            tokio::time::sleep(Duration::from_millis(100)).await;

            (call_removal_tx, removal_routine)
        };

        ActiveCalls {
            removal_routine,
            call_removal_tx,
            map,
        }
    }

    // Endpoint is {ip || port} in the network where the target wants
    // the request to be routed to, not the one from which it makes the
    // request
    // e.g. client A has {ip, 127.0.0.1 || p2p port, 5959} but when it
    // makes a request to a peer, his address may look like 127.0.0.1:9212
    pub async fn get(&self, server_endpoint: &String) -> Option<Arc<Call>> {
        let map_lock = self.map.lock().await;

        match map_lock.get(server_endpoint) {
            Some(c) => return Some(c.clone()),
            None => return None,
        }
    }

    pub async fn insert_inbound(&self, endpoint: String) -> Option<Arc<Call>> {
        let mut map = self.map.lock().await;

        return map
            .insert(endpoint.clone(), Arc::new(Call::Inbound { endpoint }));
    }

    pub async fn insert_outbound(&self, endpoint: String) -> Option<Arc<Call>> {
        let mut map = self.map.lock().await;

        return map
            .insert(endpoint.clone(), Arc::new(Call::Outbound { endpoint }));
    }

    pub fn delayed_remove(&self, endpoint: String) -> Result<(), String> {
        match self.call_removal_tx.send(endpoint) {
            Ok(_) => Ok(()),
            Err(err) => {
                return Err(format!(
                    "Delayed call removal request has been unsuccessful. \
                    Call removal channel might have been closed, err: {}",
                    err,
                ));
            }
        }
    }

    pub async fn remove(&self, endpoint: &String) -> Option<Arc<Call>> {
        let mut map = self.map.lock().await;

        map.remove(endpoint)
    }
}

impl RemovalRoutine {
    pub async fn run(&self) {
        let mut call_removal_rx = self.call_removal_rx.lock().await;

        loop {
            let endpoint = match call_removal_rx.recv().await {
                Some(r) => r,
                None => {
                    twarn!(
                        "p2p_active_calls",
                        "",
                        "Removal routine call removal channel has been closed"
                    );

                    return;
                }
            };

            let mut map = self.map.lock().await;
            map.remove(&endpoint);
        }
    }
}
