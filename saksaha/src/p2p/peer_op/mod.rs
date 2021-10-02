mod dial;
mod listen;

use super::peer::peer_store::PeerStore;
use crate::{node::task_manager::TaskManager};
use listen::Listen;
use dial::Dial;
use std::sync::Arc;
use tokio::sync::{
    mpsc::Sender as MpscSender, oneshot::Sender as OneshotSender,
};

pub struct PeerOp {
    peer_store: Arc<PeerStore>,
    dial_loop_tx: Arc<MpscSender<usize>>,
    task_mng: Arc<TaskManager>,
}

impl PeerOp {
    pub fn new(
        peer_store: Arc<PeerStore>,
        dial_loop_tx: Arc<MpscSender<usize>>,
        rpc_port: u16,
        task_mng: Arc<TaskManager>,
    ) -> PeerOp {
        let peer_op = PeerOp {
            peer_store,
            dial_loop_tx,
            task_mng,
        };

        peer_op
    }
}

impl PeerOp {
    pub async fn start(&self, peer_op_port_tx: OneshotSender<u16>) {
        let dial_loop_tx = self.dial_loop_tx.clone();

        let listen = Listen::new(dial_loop_tx, self.task_mng.clone());

        tokio::spawn(async move {
            listen.start_listening(peer_op_port_tx).await;
        });

        let dial = Dial::new(self.task_mng.clone());

        tokio::spawn(async move {
            dial.start_dialing().await;
        });
    }
}
