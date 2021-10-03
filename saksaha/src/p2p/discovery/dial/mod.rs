mod handler;

use crate::{
    msg_err,
    node::task_manager::{MsgKind, TaskManager},
    p2p::{
        address::AddressBook,
        credential::Credential,
        discovery::{dial::handler::HandleResult, whoareyou},
        peer::peer_store::PeerStore,
    },
};
use handler::Handler;
use logger::log;
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};
use tokio::sync::{mpsc::Receiver, Mutex};

pub struct Dial {
    pub address_book: Arc<AddressBook>,
    pub peer_store: Arc<PeerStore>,
    disc_port: Option<u16>,
    peer_op_port: u16,
    task_mng: Arc<TaskManager>,
    credential: Arc<Credential>,
    dial_loop_rx: Arc<Mutex<Receiver<usize>>>,
}

impl Dial {
    pub fn new(
        address_book: Arc<AddressBook>,
        peer_store: Arc<PeerStore>,
        disc_port: Option<u16>,
        peer_op_port: u16,
        task_mng: Arc<TaskManager>,
        credential: Arc<Credential>,
        dial_loop_rx: Arc<Mutex<Receiver<usize>>>,
    ) -> Dial {
        Dial {
            address_book,
            peer_store,
            disc_port,
            peer_op_port,
            task_mng,
            credential,
            dial_loop_rx,
        }
    }

    pub async fn start(&self, my_disc_port: u16) {
        let my_disc_endpoint = format!("127.0.0.1:{}", my_disc_port);
        let mut dial_loop_rx = self.dial_loop_rx.lock().await;

        loop {
            'main: loop {
                let start = SystemTime::now();

                if let Some(peer) = self.peer_store.next().await {
                    let credential = self.credential.clone();
                    let address_book = self.address_book.clone();

                    let mut handler = Handler::new(
                        peer,
                        credential,
                        self.peer_op_port,
                        address_book,
                        my_disc_endpoint.to_owned(),
                    );

                    match handler.run().await {
                        Ok(res) => {
                            if let HandleResult::AddressNotFound = res {
                                break 'main;
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
                        log!(
                            DEBUG,
                            "Error sleeping the duration, err: {}",
                            err
                        );
                    }
                }
            }

            match dial_loop_rx.recv().await {
                Some(_) => (),
                None => {
                    let msg = msg_err!(
                        MsgKind::ResourceNotAvailable,
                        "dial loop channel has been closed",
                    );

                    self.task_mng.send(msg).await;
                }
            }
        }
    }
}
