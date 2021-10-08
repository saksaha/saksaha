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

pub struct Disc {
    task_mng: Arc<TaskManager>,
}

impl Disc {
    pub fn new(task_mng: Arc<TaskManager>) -> Disc {
        Disc { task_mng }
    }

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

        let dial = Dial::new(self.task_mng.clone());
        dial.start(
            disc_port,
            peer_store.clone(),
            peer_op_port,
            credential.clone(),
            disc_wakeup_rx.clone(),
            peer_op_wakeup_tx.clone(),
        )
        .await;

        Status::Launched
    }
}
