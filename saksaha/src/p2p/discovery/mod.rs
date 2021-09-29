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
    disc_port: usize,
    bootstrap_peers: Option<Vec<String>>,
    peer_store: Arc<Mutex<PeerStore>>,
    task_mng: Arc<TaskManager>,
}

impl Disc {
    pub fn new(
        disc_port: usize,
        bootstrap_peers: Option<Vec<String>>,
        peer_store: Arc<Mutex<PeerStore>>,
        task_mng: Arc<TaskManager>,
    ) -> Self {
        let address_book = AddressBook::new();


        Disc {
            disc_port,
            bootstrap_peers,
            peer_store,
            task_mng,
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
        let dialer = dial::Dial::new(
            self.bootstrap_peers.to_owned(),
            peer_store,
            self.disc_port,
        );

        tokio::spawn(async move {
            dialer.start_dialing().await;
        });
    }
}
