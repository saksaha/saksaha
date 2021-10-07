mod dial;
mod listen;
mod status;
mod whoareyou;

use self::listen::Listen;
use crate::{
    common::{Error, Result},
    node::task_manager::TaskManager,
    p2p::{credential::Credential, peer::peer_store::PeerStore},
};
use dial::Dial;
pub use status::Status;
use std::sync::Arc;
use tokio::{
    net::TcpListener,
    sync::{
        mpsc::{Receiver, Sender},
        Mutex,
    },
};

struct Components {
    listen: Listen,
    dial: Dial,
}

pub struct Disc {
    task_mng: Arc<TaskManager>,
    // address_book: Arc<AddressBook>,
    // disc_port: Option<u16>,
    // peer_store: Arc<PeerStore>,
    // credential: Arc<Credential>,
    // disc_wakeup_rx: Arc<Mutex<Receiver<usize>>>,
    // peer_op_wakeup_tx: Arc<Sender<usize>>,
}

impl Disc {
    pub fn new(
        task_mng: Arc<TaskManager>,
        // disc_port: Option<u16>,
        // peer_store: Arc<PeerStore>,
        // credential: Arc<Credential>,
        // disc_wakeup_rx: Arc<Mutex<Receiver<usize>>>,
        // peer_op_wakeup_tx: Arc<Sender<usize>>,
        // disc_listener: TcpListener,
    ) -> Disc {
        // let address_book = Arc::new(AddressBook::new(bootstrap_urls));

        Disc {
            task_mng,
            // // address_book,
            // disc_port,
            // peer_store,
            // credential,
            // disc_wakeup_rx,
            // peer_op_wakeup_tx,
        }
    }

    // fn make_components(&self, peer_op_port: u16) -> Result<Components> {
    //     let peer_store = self.peer_store.clone();
    //     let task_mng = self.task_mng.clone();
    //     let credential = self.credential.clone();

    //     let listen = Listen::new(
    //         self.disc_port,
    //         peer_op_port,
    //         peer_store,
    //         task_mng,
    //         credential,
    //     );

    //     let peer_store = self.peer_store.clone();
    //     let task_mng = self.task_mng.clone();
    //     let credential = self.credential.clone();

    //     let dial = Dial::new(
    //         peer_store,
    //         peer_op_port,
    //         task_mng,
    //         credential,
    //         self.disc_wakeup_rx.clone(),
    //         self.peer_op_wakeup_tx.clone(),
    //     );

    //     let components = Components { listen, dial };

    //     Ok(components)
    // }

    pub async fn start(
        &self,
        peer_op_port: u16,
        peer_store: Arc<PeerStore>,
        credential: Arc<Credential>,
        disc_wakeup_rx: Arc<Mutex<Receiver<usize>>>,
        peer_op_wakeup_tx: Arc<Sender<usize>>,
        disc_listener: TcpListener,
        disc_port: u16,
    ) -> Status<Error> {
        let listen = Listen::new();
        let listen_started = listen.start(
            disc_listener,
            peer_op_port,
            peer_store.clone(),
            self.task_mng.clone(),
            credential.clone(),
        );

        match listen_started.await {
            Ok(_) => (),
            Err(err) => return Status::SetupFailed(err.into()),
        };

        let dial = Dial::new(
            self.task_mng.clone(),
        );
        dial.start(disc_port,
            peer_store.clone(),
            peer_op_port,
            credential.clone(),
            disc_wakeup_rx.clone(),
            peer_op_wakeup_tx.clone(),
        ).await;

        Status::Launched
    }
}
