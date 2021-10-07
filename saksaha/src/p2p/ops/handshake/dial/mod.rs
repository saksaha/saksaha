mod routine;

use crate::{
    common::Result,
    msg_err, msg_errd,
    node::task_manager::{MsgKind, TaskManager},
    p2p::{
        credential::Credential, ops::handshake::dial::routine::Routine,
        peer::peer_store::PeerStore,
    },
};
use logger::log;
use std::{sync::Arc, time::Duration};
use tokio::sync::{
    mpsc::{Receiver, Sender},
    Mutex,
};

pub struct Dial {
    task_mng: Arc<TaskManager>,
}

impl Dial {
    pub fn new(task_mng: Arc<TaskManager>) -> Dial {
        Dial { task_mng }
    }

    pub async fn start(
        &self,
        credential: Arc<Credential>,
        disc_wakeup_tx: Arc<Sender<usize>>,
        peer_op_wakeup_rx: Arc<Mutex<Receiver<usize>>>,
        peer_store: Arc<PeerStore>,
    ) -> Result<()> {
        log!(DEBUG, "Start dial - handshake\n");

        let task_mng = self.task_mng.clone();
        let routine = Routine::new(peer_store.clone(), credential.clone());
        routine.run();

        tokio::spawn(async move {
            loop {
                let mut peer_op_wakeup_rx = peer_op_wakeup_rx.lock().await;

                match peer_op_wakeup_rx.recv().await {
                    Some(_) => routine.wakeup().await,
                    None => {
                        let msg = msg_err!(
                            MsgKind::SetupFailure,
                            "Cannot receive peer op \
                            wake up msg. Is channel closed?",
                        );

                        task_mng.send(msg).await;
                    }
                }
            }
        });

        Ok(())
    }
}
