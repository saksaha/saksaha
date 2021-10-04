mod dial;
mod listen;
mod status;

pub use self::status::Status;
use super::peer::peer_store::PeerStore;
use crate::{
    common::{Error, Result},
    err,
    node::task_manager::TaskManager,
};
use dial::Dial;
use listen::Listen;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;

struct Components {
    dial: Dial,
    listen: Listen,
}

pub struct PeerOp {
    peer_store: Arc<PeerStore>,
    dial_wakeup_tx: Arc<Sender<usize>>,
    task_mng: Arc<TaskManager>,
}

impl PeerOp {
    pub fn new(
        peer_store: Arc<PeerStore>,
        dial_wakeup_tx: Arc<Sender<usize>>,
        rpc_port: u16,
        task_mng: Arc<TaskManager>,
    ) -> PeerOp {
        let peer_op = PeerOp {
            peer_store,
            dial_wakeup_tx,
            task_mng,
        };

        peer_op
    }

    fn make_components(&self) -> Result<Components> {
        let listen =
            Listen::new(self.dial_wakeup_tx.clone(), self.task_mng.clone());
        let dial = Dial::new(self.task_mng.clone(), self.dial_wakeup_tx.clone());

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
            dial.start_dialing().await;
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
