mod handler;
mod routine;

use crate::{
    msg_err, msg_errd,
    node::{msg::Kind, task_manager::TaskManager},
    p2p::{credential::Credential, peer::peer_store::PeerStore},
};
use handler::Handler;
use logger::log;
use routine::Routine;
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};
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
        my_disc_port: u16,
        peer_store: Arc<PeerStore>,
        peer_op_port: u16,
        credential: Arc<Credential>,
        disc_wakeup_rx: Arc<Mutex<Receiver<usize>>>,
        peer_op_wakeup_tx: Arc<Sender<usize>>,
    ) {
        let task_mng = self.task_mng.clone();

        let routine = Arc::new(Routine::new(
            peer_store.clone(),
            credential.clone(),
            peer_op_port,
            my_disc_port,
            peer_op_wakeup_tx.clone(),
        ));

        let routine_clone = routine.clone();
        routine_clone.run();

        let routine_clone = routine.clone();
        let disc_wakeup_rx = disc_wakeup_rx.clone();
        tokio::spawn(async move {
            loop {
                let mut disc_wakeup_rx = disc_wakeup_rx.lock().await;

                match disc_wakeup_rx.recv().await {
                    Some(_) => {
                        routine_clone.wakeup().await;
                    }
                    None => {
                        let msg = msg_err!(
                            Kind::SetupFailure,
                            "Cannot receive disc dial wakeup msg, \
                            is channel closed?",
                        );

                        task_mng.send(msg).await;

                        tokio::time::sleep(Duration::from_millis(1000)).await;
                    }
                };
            }
        });
    }
}
