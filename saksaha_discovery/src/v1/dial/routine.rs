use super::handler::{HandleStatus, Handler};
use logger::log;
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};
use tokio::sync::{mpsc::Sender, Mutex};

pub struct Routine {
    // peer_store: Arc<PeerStore>,
    // credential: Arc<Credential>,
    peer_op_port: u16,
    is_running: Arc<Mutex<bool>>,
    disc_port: u16,
    last_peer_idx: Arc<Mutex<usize>>,
}

impl Routine {
    pub fn new(
        // peer_store: Arc<PeerStore>,
        // credential: Arc<Credential>,
        peer_op_port: u16,
        disc_port: u16,
    ) -> Routine {
        let is_running = Arc::new(Mutex::new(false));

        Routine {
            // peer_store,
            // credential,
            peer_op_port,
            last_peer_idx: Arc::new(Mutex::new(0)),
            is_running,
            disc_port,
        }
    }

    pub fn run(&self) {
        log!(DEBUG, "Start dial - disc\n");

        // let peer_store = self.peer_store.clone();
        // let credential = self.credential.clone();
        let is_running = self.is_running.clone();
        let peer_op_port = self.peer_op_port;
        let last_peer_idx = self.last_peer_idx.clone();
        let disc_port = self.disc_port;

        tokio::spawn(async move {
            let mut is_running_lock = is_running.lock().await;
            *is_running_lock = true;
            std::mem::drop(is_running_lock);

            loop {
                let start = SystemTime::now();

                let handler = Handler::new(
                    // peer_store.clone(),
                    // credential.clone(),
                    peer_op_port,
                    disc_port,
                    last_peer_idx.clone(),
                );

                match handler.run().await {
                    HandleStatus::IllegalEndpoint(err) => {
                        log!(
                            DEBUG,
                            "Peer may have an illegal endpoint, err: {}\n",
                            err
                        );
                    }
                    HandleStatus::NoAvailablePeer => {
                        log!(DEBUG, "No available peer to discover\n");

                        break;
                    }
                    HandleStatus::IllegalPeerFound(idx) => {
                        log!(
                            DEBUG,
                            "Illegal peer has been found, idx: {}\n",
                            idx,
                        );
                    }
                    HandleStatus::ConnectionFail(err) => {
                        log!(
                            DEBUG,
                            "Disc dial connection fail, err: {}\n",
                            err
                        );
                    }
                    HandleStatus::LocalAddrIdentical => (),
                    HandleStatus::Success(_) => (),
                    HandleStatus::WhoAreYouInitiateFail(err) => {
                        log!(
                            DEBUG,
                            "Disc dial who are you \
                                initiate failed, err: {}\n",
                            err
                        );
                    }
                    HandleStatus::WhoAreYouAckReceiveFail(err) => {
                        log!(
                            DEBUG,
                            "Disc dial who are you \
                                ack receive failed, err: {}\n",
                            err
                        );
                    }
                    HandleStatus::PeerUpdateFail(err) => {
                        log!(
                            DEBUG,
                            "Disc dial peer update fail, err: {}\n",
                            err
                        );
                    }
                }

                tokio::time::sleep(Duration::from_millis(1000)).await;

                match start.elapsed() {
                    Ok(_) => (),
                    Err(err) => {
                        log!(
                            DEBUG,
                            "Error sleeping the duration, err: {}\n",
                            err
                        );
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
            log!(DEBUG, "Disc dial routine is not running, waking up\n");

            self.run();
        }
    }
}