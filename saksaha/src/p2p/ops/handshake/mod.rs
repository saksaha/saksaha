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
    credential: Arc<Credential>,
    peer_store: Arc<PeerStore>,
    disc_wakeup_tx: Arc<Sender<usize>>,
    rpc_port: u16,
    task_mng: Arc<TaskManager>,
    peer_op_wakeup_rx: Arc<Mutex<Receiver<usize>>>,
}

impl Handshake {
    pub fn new(
        peer_store: Arc<PeerStore>,
        disc_wakeup_tx: Arc<Sender<usize>>,
        rpc_port: u16,
        task_mng: Arc<TaskManager>,
        peer_op_wakeup_rx: Arc<Mutex<Receiver<usize>>>,
        credential: Arc<Credential>,
        peer_op_listener: TcpListener,
    ) -> Handshake {
        let peer_op = Handshake {
            credential,
            peer_store,
            disc_wakeup_tx,
            rpc_port,
            task_mng,
            peer_op_wakeup_rx,
        };

        peer_op
    }

    fn make_components(&self) -> Result<Components> {
        let listen = Listen::new(
            self.disc_wakeup_tx.clone(),
            self.task_mng.clone(),
            self.credential.clone(),
        );

        let dial = Dial::new(
            self.credential.clone(),
            self.task_mng.clone(),
            self.disc_wakeup_tx.clone(),
            self.peer_op_wakeup_rx.clone(),
            self.peer_store.clone(),
        );

        let components = Components { dial, listen };

        Ok(components)
    }

    async fn start_components(&self, components: Components) -> Result<u16> {
        let listen = components.listen;
        let listen_start = tokio::spawn(async move {
            return listen.start().await;
        });

        let peer_op_port = match listen_start.await {
            Ok(res) => match res {
                Ok(port) => port,
                Err(err) => return Err(err),
            },
            Err(err) => return Err(err.into()),
        };

        let dial = components.dial;
        tokio::spawn(async move {
            dial.start().await;
        });

        Ok(peer_op_port)
    }

    pub async fn start(&self) -> Status<u16, Error> {
        let components = match self.make_components() {
            Ok(c) => c,
            Err(err) => return Status::SetupFailed(err),
        };

        let peer_op_port = match self.start_components(components).await {
            Ok(port) => port,
            Err(err) => return Status::SetupFailed(err),
        };

        Status::Launched(peer_op_port)
    }
}
