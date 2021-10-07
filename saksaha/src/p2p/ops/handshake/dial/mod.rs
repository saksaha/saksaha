mod routine;

use crate::{
    msg_errd,
    node::task_manager::TaskManager,
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
    disc_wakeup_tx: Arc<Sender<usize>>,
    peer_op_wakeup_rx: Arc<Mutex<Receiver<usize>>>,
    peer_store: Arc<PeerStore>,
    credential: Arc<Credential>,
}

impl Dial {
    pub fn new(
        credential: Arc<Credential>,
        task_mng: Arc<TaskManager>,
        disc_wakeup_tx: Arc<Sender<usize>>,
        peer_op_wakeup_rx: Arc<Mutex<Receiver<usize>>>,
        peer_store: Arc<PeerStore>,
    ) -> Dial {
        Dial {
            credential,
            task_mng,
            disc_wakeup_tx,
            peer_op_wakeup_rx,
            peer_store,
        }
    }

    pub async fn start(self) {
        log!(DEBUG, "Start peer op dialing\n");

        let routine =
            Routine::new(self.peer_store.clone(), self.credential.clone());
        routine.run();

        // tokio::time::sleep(Duration::from_millis(4000)).await;

        // println!("peer op dial woke up");

        // match self.dial_wakeup_tx.send(0).await {
        //     Ok(_) => {
        //         println!("peer op dial start sent!");
        //     },
        //     Err(err) => {
        //         println!("peer op dial start send fail, err: {}", err);
        //     }
        // };

        tokio::spawn(async move {
            loop {
                let mut peer_op_wakeup_rx = self.peer_op_wakeup_rx.lock().await;
                match peer_op_wakeup_rx.recv().await {
                    Some(_) => routine.wakeup().await,
                    None => {
                        let msg = msg_errd!(
                            "Cannot receive peer op \
                            wake up msg. Is channel closed?",
                        );

                        self.task_mng.send(msg).await;
                    }
                }
            }
        });
    }
}
