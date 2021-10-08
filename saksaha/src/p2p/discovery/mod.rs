mod dialer;
mod listener;
mod status;
mod whoareyou;

use self::listener::Listener;
use crate::{
    common::{Error, Result},
    node::task_manager::TaskManager,
    p2p::{credential::Credential, peer::peer_store::PeerStore},
};
use dialer::Dialer;
pub use status::Status;
use std::sync::Arc;
use tokio::{
    net::TcpListener,
    sync::{
        mpsc::{Receiver, Sender},
        Mutex,
    },
};

pub struct Disc {}

impl Disc {
    pub fn new() -> Disc {
        Disc {}
    }

    pub async fn start(
        &self,
        port: Option<u16>,
        p2p_listener_port: u16,
        peer_store: Arc<PeerStore>,
        credential: Arc<Credential>,
        peer_op_wakeup_tx: Arc<Sender<usize>>,
    ) -> Status<Error> {
        let listener = Listener::new();
        let listener_port = match listener
            .start(
                port,
                p2p_listener_port,
                peer_store.clone(),
                credential.clone(),
            )
            .await
        {
            listener::Status::Launched(port) => port,
            listener::Status::SetupFailed(err) => {
                return Status::SetupFailed(err)
            }
        };

        let dialer = Dialer::new();
        match dialer
            .start(
                listener_port,
                peer_store.clone(),
                p2p_listener_port,
                credential.clone(),
                peer_op_wakeup_tx.clone(),
            )
            .await
        {
            Ok(_) => (),
            Err(err) => return Status::SetupFailed(err),
        };

        Status::Launched
    }
}
