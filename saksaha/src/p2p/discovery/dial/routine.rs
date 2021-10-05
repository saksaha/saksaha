use logger::log;
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};
use tokio::sync::{Mutex, mpsc::Sender};

use super::handler::Handler;
use crate::p2p::{address::address_book::AddressBook, credential::Credential, discovery::dial::handler::HandleStatus, peer::peer_store::PeerStore};

pub struct Routine {
    peer_store: Arc<PeerStore>,
    credential: Arc<Credential>,
    address_book: Arc<AddressBook>,
    peer_op_port: u16,
    is_running: Arc<Mutex<bool>>,
    my_disc_endpoint: String,
    peer_op_wakeup_tx: Arc<Sender<usize>>,
}

impl Routine {
    pub fn new(
        peer_store: Arc<PeerStore>,
        credential: Arc<Credential>,
        address_book: Arc<AddressBook>,
        peer_op_port: u16,
        disc_port: u16,
        peer_op_wakeup_tx: Arc<Sender<usize>>,
    ) -> Routine {
        let is_running = Arc::new(Mutex::new(false));
        let my_disc_endpoint = format!("127.0.0.1:{}", disc_port);

        Routine {
            peer_store,
            credential,
            address_book,
            peer_op_port,
            is_running,
            my_disc_endpoint,
            peer_op_wakeup_tx,
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
        let peer_op_wake_tx = self.peer_op_wakeup_tx.clone();

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
                        peer_op_wake_tx.clone(),
                    );

                    match handler.run().await {
                        HandleStatus::NoAvailableAddress => {
                            break;
                        }
                        HandleStatus::ConnectionFail(err) => {
                            log!(
                                DEBUG,
                                "Disc dial connection fail, err: {}\n",
                                err
                            );
                        }
                        HandleStatus::LocalAddrIdentical => (),
                        HandleStatus::Success => (),
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
                } else {
                    log!(DEBUG, "Peer not available");

                    tokio::time::sleep(Duration::from_millis(1000)).await;
                }

                tokio::time::sleep(Duration::from_millis(1000)).await;

                match start.elapsed() {
                    Ok(_) => (),
                    Err(err) => {
                        log!(
                            DEBUG,
                            "Error sleeping the duration, err: {}",
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
