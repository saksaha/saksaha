mod dial;
mod listen;
mod whoareyou;
mod status;

pub use status::Status;
use dial::Dial;
use self::listen::Listen;
use super::{
    address::AddressBook, credential::Credential, peer::peer_store::PeerStore,
};
use crate::{common::{Error, Result}, node::task_manager::TaskManager};
use std::sync::Arc;
use tokio::sync::{mpsc::Receiver, Mutex};

pub struct Components {
    listen: Listen,
    dial: Dial,
}

pub struct Disc {
    address_book: Arc<AddressBook>,
    disc_port: Option<u16>,
    peer_store: Arc<PeerStore>,
    task_mng: Arc<TaskManager>,
    credential: Arc<Credential>,
    dial_loop_rx: Arc<Mutex<Receiver<usize>>>,
}

impl Disc {
    pub fn new(
        disc_port: Option<u16>,
        bootstrap_urls: Option<Vec<String>>,
        peer_store: Arc<PeerStore>,
        task_mng: Arc<TaskManager>,
        credential: Arc<Credential>,
        dial_loop_rx: Arc<Mutex<Receiver<usize>>>,
    ) -> Disc {
        let address_book = Arc::new(AddressBook::new(bootstrap_urls));

        Disc {
            address_book,
            disc_port,
            peer_store,
            task_mng,
            credential,
            dial_loop_rx,
        }
    }

    pub fn make_components(&self, peer_op_port: u16) -> Result<Components> {
        let peer_store = self.peer_store.clone();
        let task_mng = self.task_mng.clone();
        let credential = self.credential.clone();

        let listen = Listen::new(
            self.disc_port,
            peer_op_port,
            peer_store,
            task_mng,
            credential,
        );

        let peer_store = self.peer_store.clone();
        let address_book = self.address_book.clone();
        let task_mng = self.task_mng.clone();
        let credential = self.credential.clone();

        let dial = Dial::new(
            address_book,
            peer_store,
            self.disc_port,
            peer_op_port,
            task_mng,
            credential,
            self.dial_loop_rx.clone(),
        );

        let components = Components {
            listen,
            dial,
        };

        Ok(components)
    }

    pub async fn start(&self, peer_op_port: u16) -> Status<Error> {
        let components = match self.make_components(peer_op_port) {
            Ok(c) => c,
            Err(err) => return Status::SetupFailed(err)
        };

        let listen = components.listen;
        let listen_start = tokio::spawn(async move {
            return listen.start().await;
        });

        let disc_port: u16 = match listen_start.await {
            Ok(l) => match l {
                Ok(port) => port,
                Err(err) => return Status::SetupFailed(err)
            },
            Err(err) => return Status::SetupFailed(err.into())
        };

        let dial = components.dial;
        tokio::spawn(async move {
            dial.start(disc_port).await;
        });

        Status::Launched
    }
}
