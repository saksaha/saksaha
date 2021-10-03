mod dial;
mod listen;
mod status;

pub use self::status::Status;

use super::peer::peer_store::PeerStore;
use crate::{common::{Error, Result}, err, node::{task_manager::TaskManager}};
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
    pub async fn start(&self) -> Status<u16, Error> {
        let dial_loop_tx = self.dial_loop_tx.clone();

        let listen = match Listen::new(dial_loop_tx, self.task_mng.clone())
            .await
        {
            Ok(l) => l,
            Err(err) => {
                return Status::SetupFailed(err);
            }
        };

        // let peer_op_port = listen.port;

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

        let dial = Dial::new(self.task_mng.clone());

        tokio::spawn(async move {
            dial.start_dialing().await;
            println!("223");
        });

        Status::Launched(peer_op_port)
    }
}
