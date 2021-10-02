mod dial;
mod listen;
mod whoareyou;

use self::listen::Listen;
use super::{
    address::AddressBook, credential::Credential, peer::peer_store::PeerStore,
};
use crate::{common::SakResult, err_res, node::task_manager::TaskManager};
use logger::log;
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc::{Receiver}};

pub struct Disc {
    address_book: Arc<AddressBook>,
    disc_port: u16,
    peer_op_port: u16,
    peer_store: Arc<PeerStore>,
    task_mng: Arc<TaskManager>,
    credential: Arc<Credential>,
    dial_loop_rx: Arc<Mutex<Receiver<usize>>>,
}

impl Disc {
    pub fn new(
        disc_port: u16,
        peer_op_port: u16,
        bootstrap_urls: Option<Vec<String>>,
        peer_store: Arc<PeerStore>,
        task_mng: Arc<TaskManager>,
        credential: Arc<Credential>,
        dial_loop_rx: Arc<Mutex<Receiver<usize>>>,
    ) -> Self {
        let address_book = Arc::new(AddressBook::new(bootstrap_urls));

        Disc {
            address_book,
            disc_port,
            peer_op_port,
            peer_store,
            task_mng,
            credential,
            dial_loop_rx,
        }
    }

    pub async fn start(&self) {
        let peer_store = self.peer_store.clone();
        let task_mng = self.task_mng.clone();
        let credential = self.credential.clone();

        let listen = Listen::new(
            self.disc_port,
            self.peer_op_port,
            peer_store,
            task_mng,
            credential,
        );

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
            self.peer_op_port,
            task_mng,
            credential,
            self.dial_loop_rx.clone(),
        );

        tokio::spawn(async move {
            dialer.start_dialing().await;
        });
    }
}
