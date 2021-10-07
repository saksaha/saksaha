use crate::p2p::{
    credential::Credential,
    peer::peer_store::{Filter, PeerStore},
};
use logger::log;
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};
use tokio::sync::Mutex;

pub struct Routine {
    peer_store: Arc<PeerStore>,
    credential: Arc<Credential>,
    is_running: Arc<Mutex<bool>>,
}

impl Routine {
    pub fn new(
        peer_store: Arc<PeerStore>,
        credential: Arc<Credential>,
    ) -> Routine {
        Routine {
            is_running: Arc::new(Mutex::new(false)),
            credential,
            peer_store,
        }
    }

    pub fn run(&self) {
        let is_running = self.is_running.clone();
        let peer_store = self.peer_store.clone();
        let credential = self.credential.clone();

        tokio::spawn(async move {
            let mut is_running_lock = is_running.lock().await;
            *is_running_lock = true;
            std::mem::drop(is_running_lock);

            loop {
                let start = SystemTime::now();

                // match peer_store.next(&Filter::discovery_success).await {
                //     Some(p) => {
                //         let handler = Handler::new(p, credential.clone());

                //         match handler.run().await {
                //             HandleStatus::ConnectionFail(err) => {}
                //             HandleStatus::HandshakeInitiateFail(err) => {}
                //             HandleStatus::Success => (),
                //         };
                //     }
                //     None => {
                //         log!(DEBUG, "Cannot find any discovered peer\n");

                //         tokio::time::sleep(Duration::from_millis(2000)).await;
                //     }
                // }

                // tokio::time::sleep(Duration::from_millis(1000)).await;

                // match start.elapsed() {
                //     Ok(_) => (),
                //     Err(err) => {
                //         log!(
                //             DEBUG,
                //             "Error sleeping the duration, err: {}",
                //             err
                //         );
                //     }
                // }
            }
        });
    }

    pub async fn wakeup(&self) {}
}
