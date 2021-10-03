mod dial;
mod listen;

use super::peer::peer_store::PeerStore;
use crate::{common::Result, err_res, node::task_manager::TaskManager};
use dial::Dial;
use listen::Listen;
use std::sync::Arc;
use tokio::sync::mpsc::Sender as MpscSender;

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
    pub async fn start(
        &self,
        // peer_store: Arc<PeerStore>,
        // dial_loop_tx: Arc<MpscSender<usize>>,
        // rpc_port: u16,
        // task_mng: Arc<TaskManager>,
    ) -> Result<u16> {
        let dial_loop_tx = self.dial_loop_tx.clone();

        let listen =
            match Listen::new(dial_loop_tx, self.task_mng.clone()).await {
                Ok(l) => l,
                Err(err) => {
                    return err_res!(
                        "Error initializing peer op listen, err: {}",
                        err
                    );
                }
            };

        let peer_op_port = listen.port;

        tokio::spawn(async move {
            listen.start_listening().await;
            println!("33");
        });

        let dial = Dial::new(self.task_mng.clone());

        tokio::spawn(async move {
            dial.start_dialing().await;
            println!("223");
        });

        Ok(peer_op_port)
    }
}
