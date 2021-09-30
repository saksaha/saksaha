mod dial;
mod listen;
mod whoareyou;

use self::listen::Listen;
use super::{address::AddressBook, peer_store::PeerStore};
use crate::{common::SakResult, err_res, node::task_manager::TaskManager};
use logger::log;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Disc {
    address_book: Arc<AddressBook>,
    disc_port: usize,
    peer_store: Arc<PeerStore>,
    task_mng: Arc<TaskManager>,
}

impl Disc {
    pub fn new(
        disc_port: usize,
        bootstrap_urls: Option<Vec<String>>,
        peer_store: Arc<PeerStore>,
        task_mng: Arc<TaskManager>,
        secret: String,
    ) -> Self {
        let address_book =
            Arc::new(AddressBook::new(bootstrap_urls));

        Disc {
            address_book,
            disc_port,
            peer_store,
            task_mng,
            // secret,
        }
    }

    pub async fn start(&self) {
        let peer_store = self.peer_store.clone();
        let task_mng = self.task_mng.clone();

        let listen = Listen::new(self.disc_port, peer_store, task_mng);

        tokio::spawn(async move {
            listen.start_listening().await;
        });

        let peer_store = self.peer_store.clone();
        let address_book = self.address_book.clone();
        let dialer = dial::Dial::new(address_book, peer_store, self.disc_port);

        tokio::spawn(async move {
            dialer.start_dialing().await;
        });
    }
}
