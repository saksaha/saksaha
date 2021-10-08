mod dial;
mod listen;
mod msg;
mod status;

pub use self::status::Status;
use crate::{
    common::{Error, Result},
    err,
    node::task_manager::TaskManager,
    p2p::{credential::Credential, peer::peer_store::PeerStore},
};
use dial::Dial;
use listen::Listen;
use std::sync::Arc;
use tokio::{
    net::TcpListener,
    sync::{
        mpsc::{Receiver, Sender},
        Mutex,
    },
};

pub struct Handshake {
    task_mng: Arc<TaskManager>,
}

impl Handshake {
    pub fn new(task_mng: Arc<TaskManager>) -> Handshake {
        let peer_op = Handshake { task_mng };

        peer_op
    }

    pub async fn start(
        &self,
        peer_store: Arc<PeerStore>,
        disc_wakeup_tx: Arc<Sender<usize>>,
        rpc_port: u16,
        peer_op_wakeup_rx: Arc<Mutex<Receiver<usize>>>,
        credential: Arc<Credential>,
        peer_op_listener: TcpListener,
    ) -> Status<Error> {
        let task_mng = self.task_mng.clone();

        let dial = Dial::new(task_mng.clone());

        // let listen = Listen::new();
        // let listen_started = listen.start(
        //     disc_wakeup_tx.clone(),
        //     task_mng.clone(),
        //     credential.clone(),
        //     peer_op_listener,
        // );

        // match listen_started.await {
        //     Ok(_) => (),
        //     Err(err) => return Status::SetupFailed(err),
        // };

        let dial_started = dial.start(
            credential.clone(),
            disc_wakeup_tx.clone(),
            peer_op_wakeup_rx.clone(),
            peer_store.clone(),
        );

        match dial_started.await {
            Ok(_) => (),
            Err(err) => return Status::SetupFailed(err),
        };

        Status::Launched
    }
}
