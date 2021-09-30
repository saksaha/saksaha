mod dial;
mod listen;
mod whoareyou;

use self::listen::Listen;
use super::{
    address::AddressBook, credential::Credential, peer_store::PeerStore,
};
use crate::{common::SakResult, err_res, node::task_manager::TaskManager};
use k256::SecretKey;
use logger::log;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Disc {
    address_book: Arc<AddressBook>,
    disc_port: u16,
    p2p_port: u16,
    peer_store: Arc<PeerStore>,
    task_mng: Arc<TaskManager>,
    credential: Arc<Credential>,
}

impl Disc {
    pub fn new(
        disc_port: u16,
        p2p_port: u16,
        bootstrap_urls: Option<Vec<String>>,
        peer_store: Arc<PeerStore>,
        task_mng: Arc<TaskManager>,
        credential: Arc<Credential>,
    ) -> Self {
        let address_book = Arc::new(AddressBook::new(bootstrap_urls));

        Disc {
            address_book,
            disc_port,
            p2p_port,
            peer_store,
            task_mng,
            credential,
        }
    }

    pub async fn start(&self) {
        let peer_store = self.peer_store.clone();
        let task_mng = self.task_mng.clone();
        let credential = self.credential.clone();

        let listen =
            Listen::new(self.disc_port, peer_store, task_mng, credential);

        tokio::spawn(async move {
            listen.start_listening().await;
        });

        let peer_store = self.peer_store.clone();
        let address_book = self.address_book.clone();
        let task_mng = self.task_mng.clone();
        let credential = self.credential.clone();

        let dialer = dial::Dial::new(
            address_book,
            peer_store,
            self.disc_port,
            self.p2p_port,
            task_mng,
            credential,
        );

        tokio::spawn(async move {
            dialer.start_dialing().await;
        });
    }
}
