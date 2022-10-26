use crate::MRSError;
use sak_store_interface::Session;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{
    mpsc::{self, UnboundedReceiver, UnboundedSender},
    Mutex, RwLock,
};

pub struct SessionStore {
    store: Arc<RwLock<HashMap<String, Session>>>,
    store_tx: UnboundedSender<Session>,
    store_rx: Arc<Mutex<UnboundedReceiver<Session>>>,
}

impl SessionStore {
    pub fn init() -> SessionStore {
        let store = {
            let s = HashMap::new();
            Arc::new(RwLock::new(s))
        };

        let (store_tx, store_rx) = {
            let (tx, rx) = mpsc::unbounded_channel();

            (tx, Arc::new(Mutex::new(rx)))
        };

        SessionStore {
            store,
            store_tx,
            store_rx,
        }
    }

    pub fn add_session(&self, session: Session) {
        self.store_tx.send(session);
    }

    pub fn run(&self) -> Result<(), MRSError> {
        let store_rx = self.store_rx.clone();
        let store = self.store.clone();

        tokio::spawn(async move {
            let mut store_rx_lock = store_rx.lock().await;
            loop {
                let session = match store_rx_lock.try_recv() {
                    Ok(s) => s,
                    Err(err) => {
                        panic!("session should be received, err: {}", err);
                    }
                };

                let mut store_lock = store.write().await;
                store_lock.insert(session.id.to_string(), session);
            }
        });

        Ok(())
    }
}
