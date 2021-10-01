mod dial;
mod listen;

use super::peer::peer_store::PeerStore;
use crate::{common::SakResult, err_res};
use std::sync::{Arc,};
use tokio::sync::{Mutex, oneshot::Sender};

pub struct PeerOp {
    peer_store: Arc<PeerStore>,
    peer_op_port_tx: Arc<Sender<u16>>,
}

impl PeerOp {
    pub fn new(
        peer_store: Arc<PeerStore>,
        peer_op_port_tx: Arc<Sender<u16>>,
    ) -> PeerOp {
        let peer_op = PeerOp {
            peer_store,
            peer_op_port_tx,
        };

        peer_op
    }
}

impl PeerOp {
    pub async fn start(&self) {
        let listen = listen::Listen {};
        let peer_op_port_tx = self.peer_op_port_tx.to_owned();

        tokio::spawn(async move {
            match listen.start_listening(peer_op_port_tx).await {
                Ok(_) => Ok(()),
                Err(err) => {
                    return err_res!(
                        "Error start peer op listening, err: {}",
                        err
                    );
                }
            }
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
