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
use tokio::{net::TcpListener, sync::{
    mpsc::{Receiver, Sender},
    Mutex,
}};

struct Components {
    dial: Dial,
    listen: Listen,
}

pub struct Handshake {
    task_mng: Arc<TaskManager>,
}

impl Handshake {
    pub fn new(
        task_mng: Arc<TaskManager>,
    ) -> Handshake {
        let peer_op = Handshake {
            task_mng,
        };

        peer_op
    }

    pub async fn start(&self,
        peer_store: Arc<PeerStore>,
        disc_wakeup_tx: Arc<Sender<usize>>,
        rpc_port: u16,
        peer_op_wakeup_rx: Arc<Mutex<Receiver<usize>>>,
        credential: Arc<Credential>,
        peer_op_listener: TcpListener,
    ) -> Status<u16, Error> {
        let listen = Listen::new(
            disc_wakeup_tx.clone(),
            self.task_mng.clone(),
            credential.clone(),
        );

        let dial = Dial::new(
            credential.clone(),
            self.task_mng.clone(),
            disc_wakeup_tx.clone(),
            peer_op_wakeup_rx.clone(),
            peer_store.clone(),
        );

        let components = Components { dial, listen };

        let listen = components.listen;
        let listen_start = tokio::spawn(async move {
            return listen.start().await;
        });

        let peer_op_port = match listen_start.await {
            Ok(res) => match res {
                Ok(port) => port,
                Err(err) => return Status::SetupFailed(err),
            },
            Err(err) => return Status::SetupFailed(err.into()),
        };

        let dial = components.dial;
        tokio::spawn(async move {
            dial.start().await;
        });

        Status::Launched(peer_op_port)
    }

    // pub async fn start(&self) -> Status<u16, Error> {
    //     // let components = match self.make_components() {
    //     //     Ok(c) => c,
    //     //     Err(err) => return Status::SetupFailed(err),
    //     // };

    //     let peer_op_port = match self.start_components().await {
    //         Ok(port) => port,
    //         Err(err) => return Status::SetupFailed(err),
    //     };

    //     Status::Launched(peer_op_port)
    // }
}
