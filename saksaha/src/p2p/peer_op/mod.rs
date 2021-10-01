mod dial;
mod listen;

use super::peer::peer_store::PeerStore;
use crate::{common::SakResult, err_res};
use std::sync::Arc;
use tokio::sync::{mpsc::Sender, Mutex};

pub struct PeerOp {
    peer_store: Arc<PeerStore>,
    peer_op_port_tx: Arc<Sender<u16>>,
    dial_loop_tx: Arc<Sender<usize>>,
}

impl PeerOp {
    pub fn new(
        peer_store: Arc<PeerStore>,
        peer_op_port_tx: Arc<Sender<u16>>,
        dial_loop_tx: Arc<Sender<usize>>,
    ) -> PeerOp {
        let peer_op = PeerOp {
            peer_store,
            peer_op_port_tx,
            dial_loop_tx,
        };

        peer_op
    }
}

impl PeerOp {
    pub async fn start(&self) {
        let peer_op_port_tx = self.peer_op_port_tx.clone();
        let dial_loop_tx = self.dial_loop_tx.clone();

        let listen = listen::Listen::new(peer_op_port_tx, dial_loop_tx);

        tokio::spawn(async move {
            listen.start_listening().await;
        });

        let dial = dial::Dial {};

        tokio::spawn(async move {
            match dial.start_dialing().await {
                Ok(_) => Ok(()),
                Err(err) => {
                    return err_res!(
                        "Error start peer op dialing, err: {}",
                        err
                    );
                }
            }
        });
    }
}
