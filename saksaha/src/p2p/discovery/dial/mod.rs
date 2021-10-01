mod handler;

use super::Disc;
use crate::{
    node::task_manager::TaskManager,
    p2p::{
        address::AddressBook, credential::Credential, discovery::whoareyou,
        peer::peer_store::PeerStore,
    },
};
use handler::Handler;
use logger::log;
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};

pub struct Dial {
    pub address_book: Arc<AddressBook>,
    pub peer_store: Arc<PeerStore>,
    disc_port: u16,
    peer_op_port: u16,
    task_mng: Arc<TaskManager>,
    credential: Arc<Credential>,
}

impl Dial {
    pub fn new(
        address_book: Arc<AddressBook>,
        peer_store: Arc<PeerStore>,
        disc_port: u16,
        peer_op_port: u16,
        task_mng: Arc<TaskManager>,
        credential: Arc<Credential>,
    ) -> Dial {
        Dial {
            address_book,
            peer_store,
            disc_port,
            peer_op_port,
            task_mng,
            credential,
        }
    }

    pub async fn start_dialing(&self) {
        let my_disc_endpoint = format!("127.0.0.1:{}", self.disc_port);

        loop {
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
                    Ok(_) => (),
                    Err(err) => {
                        log!(DEBUG, "Error processing request, err: {}\n", err,);
                    }
                }
            } else {
                println!("Peer not available");
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
    }
}
