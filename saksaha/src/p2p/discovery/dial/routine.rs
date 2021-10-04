use logger::log;
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};
use tokio::sync::Mutex;

use super::handler::Handler;
use crate::p2p::{
    address::AddressBook, credential::Credential,
    discovery::dial::handler::HandleResult, peer::peer_store::PeerStore,
};

pub struct Routine {
    peer_store: Arc<PeerStore>,
    credential: Arc<Credential>,
    address_book: Arc<AddressBook>,
    peer_op_port: u16,
    disc_port: u16,
    is_running: Arc<Mutex<bool>>,
    my_disc_endpoint: String,
}

impl Routine {
    pub fn new(
        peer_store: Arc<PeerStore>,
        credential: Arc<Credential>,
        address_book: Arc<AddressBook>,
        peer_op_port: u16,
        disc_port: u16,
    ) -> Routine {
        let is_running = Arc::new(Mutex::new(false));
        let my_disc_endpoint = format!("127.0.0.1:{}", disc_port);

        Routine {
            peer_store,
            credential,
            address_book,
            peer_op_port,
            disc_port,
            is_running,
            my_disc_endpoint,
        }
    }

    pub fn run(&self) {
        log!(DEBUG, "Start disc dial\n");

        let peer_store = self.peer_store.clone();
        let credential = self.credential.clone();
        let address_book = self.address_book.clone();
        let is_running = self.is_running.clone();
        let peer_op_port = self.peer_op_port;
        let my_disc_endpoint = self.my_disc_endpoint.to_owned();

        tokio::spawn(async move {
            let mut is_running_lock = is_running.lock().await;
            *is_running_lock = true;
            std::mem::drop(is_running_lock);

            loop {
                let start = SystemTime::now();

                if let Some(peer) = peer_store.next().await {
                    let mut handler = Handler::new(
                        peer,
                        credential.clone(),
                        peer_op_port,
                        address_book.clone(),
                        my_disc_endpoint.to_owned(),
                    );

                    match handler.run().await {
                        Ok(res) => {
                            if let HandleResult::AddressNotFound = res {
                                break;
                            }
                        }
                        Err(err) => {
                            log!(
                                DEBUG,
                                "Error processing request, err: {}\n",
                                err,
                            );
                        }
                    }
                } else {
                    log!(DEBUG, "Peer not available");

                    tokio::time::sleep(Duration::from_millis(1000)).await;
                }

                tokio::time::sleep(Duration::from_millis(1000)).await;

                match start.elapsed() {
                    Ok(_) => (),
                    Err(err) => {
                        log!(DEBUG, "Error sleeping the duration, err: {}", err);
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
